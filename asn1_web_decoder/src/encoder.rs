use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EncodeError {
    InvalidValue(String),
    InvalidLength(String),
    InvalidTag(String),
    UnsupportedType(String),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodeError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            EncodeError::InvalidLength(msg) => write!(f, "Invalid length: {}", msg),
            EncodeError::InvalidTag(msg) => write!(f, "Invalid tag: {}", msg),
            EncodeError::UnsupportedType(msg) => write!(f, "Unsupported type: {}", msg),
        }
    }
}

impl Error for EncodeError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asn1Node {
    pub label: String,
    pub tag_class: String,
    pub tag_number: u32,
    pub is_constructed: bool,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub length: usize,
    pub value: Option<String>,
    pub children: Option<Vec<Asn1Node>>,
}

/// Encode an ASN.1 node tree back to DER bytes
pub fn encode_asn1_tree(node: &Asn1Node) -> Result<Vec<u8>, EncodeError> {
    let mut result = Vec::new();

    // Encode tag
    let tag_byte = encode_tag(node.tag_class.as_str(), node.tag_number, node.is_constructed)?;
    result.push(tag_byte);

    // Get content bytes
    let content = if node.is_constructed && node.children.is_some() {
        // Encode all children
        let mut child_bytes = Vec::new();
        for child in node.children.as_ref().unwrap() {
            child_bytes.extend_from_slice(&encode_asn1_tree(child)?);
        }
        child_bytes
    } else if let Some(ref value) = node.value {
        // Encode primitive value
        encode_value(node.tag_number, value)?
    } else {
        Vec::new()
    };

    // Encode length
    let length_bytes = encode_length(content.len())?;
    result.extend_from_slice(&length_bytes);

    // Add content
    result.extend_from_slice(&content);

    Ok(result)
}

/// Encode the tag byte
fn encode_tag(tag_class: &str, tag_number: u32, is_constructed: bool) -> Result<u8, EncodeError> {
    let class_bits = match tag_class {
        "Universal" => 0b00000000,
        "Application" => 0b01000000,
        "Context" => 0b10000000,
        "Private" => 0b11000000,
        "PEM" => 0b00000000, // Treat PEM as Universal for encoding
        _ => return Err(EncodeError::InvalidTag(format!("Unknown tag class: {}", tag_class))),
    };

    let constructed_bit = if is_constructed { 0b00100000 } else { 0b00000000 };

    if tag_number > 30 {
        // Long form tag encoding (not implemented for simplicity)
        return Err(EncodeError::UnsupportedType("Tag numbers > 30 not yet supported".to_string()));
    }

    Ok(class_bits | constructed_bit | (tag_number as u8))
}

/// Encode the length in DER format
fn encode_length(length: usize) -> Result<Vec<u8>, EncodeError> {
    if length < 128 {
        // Short form
        Ok(vec![length as u8])
    } else {
        // Long form
        let mut length_bytes = Vec::new();
        let mut len = length;
        
        while len > 0 {
            length_bytes.insert(0, (len & 0xFF) as u8);
            len >>= 8;
        }

        let num_length_bytes = length_bytes.len();
        if num_length_bytes > 127 {
            return Err(EncodeError::InvalidLength("Length too large".to_string()));
        }

        let mut result = vec![0x80 | num_length_bytes as u8];
        result.extend_from_slice(&length_bytes);
        Ok(result)
    }
}

/// Encode a primitive value based on tag number
fn encode_value(tag_number: u32, value: &str) -> Result<Vec<u8>, EncodeError> {
    match tag_number {
        1 => encode_boolean(value),
        2 => encode_integer(value),
        3 => encode_bit_string(value),
        4 => encode_octet_string(value),
        5 => encode_null(value),
        6 => encode_object_identifier(value),
        12 | 19 | 22 | 23 | 24 => encode_string(value), // UTF8String, PrintableString, IA5String, UTCTime, GeneralizedTime
        _ => {
            // For unknown types, try to decode hex if present
            if value.starts_with("0x") {
                hex_to_bytes(&value[2..])
            } else {
                encode_string(value)
            }
        }
    }
}

fn encode_boolean(value: &str) -> Result<Vec<u8>, EncodeError> {
    match value.to_lowercase().as_str() {
        "true" | "0xff" => Ok(vec![0xFF]),
        "false" | "0x00" => Ok(vec![0x00]),
        _ => Err(EncodeError::InvalidValue(format!("Invalid boolean value: {}", value))),
    }
}

fn encode_integer(value: &str) -> Result<Vec<u8>, EncodeError> {
    // Try to parse as hex first (common for serial numbers, etc.)
    if value.starts_with("0x") {
        return hex_to_bytes(&value[2..]);
    }

    // Try to parse as decimal integer
    if let Ok(num) = value.parse::<i64>() {
        let mut bytes = Vec::new();
        let mut n = num;
        
        if n == 0 {
            return Ok(vec![0]);
        }

        let negative = n < 0;
        if negative {
            n = -n;
        }

        while n > 0 {
            bytes.insert(0, (n & 0xFF) as u8);
            n >>= 8;
        }

        // Add padding byte if high bit is set (to maintain sign)
        if bytes[0] & 0x80 != 0 && !negative {
            bytes.insert(0, 0);
        }

        if negative {
            // Two's complement
            for byte in &mut bytes {
                *byte = !*byte;
            }
            let mut carry = true;
            for byte in bytes.iter_mut().rev() {
                if carry {
                    let (new_byte, overflow) = byte.overflowing_add(1);
                    *byte = new_byte;
                    carry = overflow;
                }
            }
        }

        Ok(bytes)
    } else {
        Err(EncodeError::InvalidValue(format!("Invalid integer value: {}", value)))
    }
}

fn encode_bit_string(value: &str) -> Result<Vec<u8>, EncodeError> {
    // Expected format: "X unused bits, data: HEXSTRING"
    if let Some(data_start) = value.find("data:") {
        let hex_part = value[data_start + 5..].trim();
        let unused_bits_str = value.split_whitespace().next().unwrap_or("0");
        let unused_bits = unused_bits_str.parse::<u8>()
            .map_err(|_| EncodeError::InvalidValue("Invalid unused bits in bit string".to_string()))?;
        
        let mut result = vec![unused_bits];
        result.extend_from_slice(&hex_to_bytes(hex_part)?);
        Ok(result)
    } else {
        Err(EncodeError::InvalidValue(format!("Invalid bit string format: {}", value)))
    }
}

fn encode_octet_string(value: &str) -> Result<Vec<u8>, EncodeError> {
    hex_to_bytes(value)
}

fn encode_null(_value: &str) -> Result<Vec<u8>, EncodeError> {
    Ok(Vec::new())
}

fn encode_object_identifier(value: &str) -> Result<Vec<u8>, EncodeError> {
    // Expected format: "1.2.840.113549.1.1.11" or similar
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() < 2 {
        return Err(EncodeError::InvalidValue("OID must have at least 2 components".to_string()));
    }

    let first: u32 = parts[0].parse()
        .map_err(|_| EncodeError::InvalidValue("Invalid OID component".to_string()))?;
    let second: u32 = parts[1].parse()
        .map_err(|_| EncodeError::InvalidValue("Invalid OID component".to_string()))?;

    let mut bytes = vec![first as u8 * 40 + second as u8];

    for part in &parts[2..] {
        let num: u32 = part.parse()
            .map_err(|_| EncodeError::InvalidValue("Invalid OID component".to_string()))?;
        bytes.extend_from_slice(&encode_oid_component(num));
    }

    Ok(bytes)
}

fn encode_oid_component(mut num: u32) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }

    let mut bytes = Vec::new();
    bytes.push((num & 0x7F) as u8);
    num >>= 7;

    while num > 0 {
        bytes.insert(0, ((num & 0x7F) | 0x80) as u8);
        num >>= 7;
    }

    bytes
}

fn encode_string(value: &str) -> Result<Vec<u8>, EncodeError> {
    Ok(value.as_bytes().to_vec())
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, EncodeError> {
    let hex = hex.replace(" ", "").replace(":", "");
    let mut bytes = Vec::new();
    
    for i in (0..hex.len()).step_by(2) {
        if i + 1 < hex.len() {
            let byte_str = &hex[i..i+2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| EncodeError::InvalidValue(format!("Invalid hex string: {}", hex)))?;
            bytes.push(byte);
        }
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_tag() {
        assert_eq!(encode_tag("Universal", 2, false).unwrap(), 0x02); // INTEGER
        assert_eq!(encode_tag("Universal", 16, true).unwrap(), 0x30); // SEQUENCE
        assert_eq!(encode_tag("Context", 0, true).unwrap(), 0xA0); // [0]
    }

    #[test]
    fn test_encode_length() {
        assert_eq!(encode_length(5).unwrap(), vec![0x05]);
        assert_eq!(encode_length(127).unwrap(), vec![0x7F]);
        assert_eq!(encode_length(128).unwrap(), vec![0x81, 0x80]);
        assert_eq!(encode_length(255).unwrap(), vec![0x81, 0xFF]);
        assert_eq!(encode_length(256).unwrap(), vec![0x82, 0x01, 0x00]);
    }

    #[test]
    fn test_encode_integer() {
        assert_eq!(encode_integer("0").unwrap(), vec![0x00]);
        assert_eq!(encode_integer("127").unwrap(), vec![0x7F]);
        assert_eq!(encode_integer("128").unwrap(), vec![0x00, 0x80]);
        assert_eq!(encode_integer("255").unwrap(), vec![0x00, 0xFF]);
        assert_eq!(encode_integer("256").unwrap(), vec![0x01, 0x00]);
    }

    #[test]
    fn test_encode_oid() {
        // OID 1.2.840.113549 encodes to 0x2A 0x86 0x48 0x86 0xF7 0x0D
        assert_eq!(
            encode_object_identifier("1.2.840.113549").unwrap(),
            vec![0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D]
        );
    }
}
