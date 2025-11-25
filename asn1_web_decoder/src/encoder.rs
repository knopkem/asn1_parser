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
    let tag_class_upper = tag_class.to_uppercase();
    let class_bits = match tag_class_upper.as_str() {
        "UNIVERSAL" => 0b00000000,
        "APPLICATION" => 0b01000000,
        "CONTEXT" => 0b10000000,
        "PRIVATE" => 0b11000000,
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
        if num == 0 {
            return Ok(vec![0]);
        }

        if num > 0 {
            // Positive number encoding
            let mut bytes = Vec::new();
            let mut n = num;
            
            while n > 0 {
                bytes.insert(0, (n & 0xFF) as u8);
                n >>= 8;
            }

            // Add padding byte if high bit is set (to maintain sign)
            if bytes[0] & 0x80 != 0 {
                bytes.insert(0, 0);
            }

            Ok(bytes)
        } else {
            // Negative number encoding using two's complement
            let mut bytes = Vec::new();
            let mut n = num;
            
            // Convert to bytes (little-endian style processing)
            while n != -1 || (bytes.is_empty() || bytes[0] & 0x80 == 0) {
                bytes.insert(0, (n & 0xFF) as u8);
                n >>= 8;
            }

            Ok(bytes)
        }
    } else {
        Err(EncodeError::InvalidValue(format!("Invalid integer value: {}", value)))
    }
}

fn encode_bit_string(value: &str) -> Result<Vec<u8>, EncodeError> {
    // Expected format from decoder: "01010101... (unused bits: X)"
    // Also support legacy format: "X unused bits, data: HEXSTRING"
    
    // Try new format first: "bits (unused bits: N)"
    if let Some(unused_start) = value.rfind("(unused bits:") {
        let bits_part = value[..unused_start].trim();
        let unused_part = &value[unused_start + 13..]; // Skip "(unused bits:"
        let unused_bits_str = unused_part.trim_end_matches(')').trim();
        let unused_bits = unused_bits_str.parse::<u8>()
            .map_err(|_| EncodeError::InvalidValue("Invalid unused bits in bit string".to_string()))?;
        
        // Convert bit string to bytes
        let mut bytes = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;
        
        for ch in bits_part.chars() {
            if ch == '0' || ch == '1' {
                current_byte = (current_byte << 1) | (if ch == '1' { 1 } else { 0 });
                bit_count += 1;
                
                if bit_count == 8 {
                    bytes.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
        }
        
        // Handle remaining bits
        if bit_count > 0 {
            current_byte <<= 8 - bit_count;
            bytes.push(current_byte);
        }
        
        let mut result = vec![unused_bits];
        result.extend_from_slice(&bytes);
        Ok(result)
    } else if let Some(data_start) = value.find("data:") {
        // Legacy format: "X unused bits, data: HEXSTRING"
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
    // Handle different formats from decoder:
    // 1. "[N bytes] HEX HEX HEX..." - binary data
    // 2. Plain string - UTF-8 strings (quotes removed from decoder)
    // 3. Plain hex string - raw hex
    
    let value = value.trim();
    
    // Check if it has the "[N bytes] HEX" format
    if value.starts_with('[') {
        if let Some(hex_start) = value.find(']') {
            let hex_part = value[hex_start + 1..].trim();
            return hex_to_bytes(hex_part);
        }
    }
    
    // Try to parse as hex first - if it fails, treat as UTF-8 string
    match hex_to_bytes(value) {
        Ok(bytes) => Ok(bytes),
        Err(_) => Ok(value.as_bytes().to_vec()),
    }
}

fn encode_null(_value: &str) -> Result<Vec<u8>, EncodeError> {
    Ok(Vec::new())
}

fn encode_object_identifier(value: &str) -> Result<Vec<u8>, EncodeError> {
    // Expected format: "1.2.840.113549.1.1.11" or "1.2.840.113549.1.1.11 (SHA-256 with RSA)"
    // Strip the human-readable name if present
    let oid_str = if let Some(paren_pos) = value.find(" (") {
        value[..paren_pos].trim()
    } else {
        value.trim()
    };
    
    let parts: Vec<&str> = oid_str.split('.').collect();
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
    Ok(value.trim().as_bytes().to_vec())
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
    fn test_encode_tag_universal() {
        assert_eq!(encode_tag("Universal", 2, false).unwrap(), 0x02); // INTEGER
        assert_eq!(encode_tag("Universal", 16, true).unwrap(), 0x30); // SEQUENCE
        assert_eq!(encode_tag("Universal", 4, false).unwrap(), 0x04); // OCTET STRING
        assert_eq!(encode_tag("Universal", 3, false).unwrap(), 0x03); // BIT STRING
        assert_eq!(encode_tag("Universal", 6, false).unwrap(), 0x06); // OID
        assert_eq!(encode_tag("UNIVERSAL", 2, false).unwrap(), 0x02); // Case insensitive
    }

    #[test]
    fn test_encode_tag_context() {
        assert_eq!(encode_tag("Context", 0, true).unwrap(), 0xA0); // [0]
        assert_eq!(encode_tag("Context", 1, true).unwrap(), 0xA1); // [1]
        assert_eq!(encode_tag("Context", 3, false).unwrap(), 0x83); // [3] primitive
        assert_eq!(encode_tag("CONTEXT", 0, true).unwrap(), 0xA0); // Case insensitive
    }

    #[test]
    fn test_encode_tag_application() {
        assert_eq!(encode_tag("Application", 0, false).unwrap(), 0x40);
        assert_eq!(encode_tag("Application", 5, true).unwrap(), 0x65);
    }

    #[test]
    fn test_encode_tag_private() {
        assert_eq!(encode_tag("Private", 0, false).unwrap(), 0xC0);
        assert_eq!(encode_tag("Private", 10, true).unwrap(), 0xEA);
    }

    #[test]
    fn test_encode_tag_pem_as_universal() {
        assert_eq!(encode_tag("PEM", 16, true).unwrap(), 0x30); // PEM treated as Universal
    }

    #[test]
    fn test_encode_tag_invalid() {
        assert!(encode_tag("Unknown", 2, false).is_err());
    }

    #[test]
    fn test_encode_length_short_form() {
        assert_eq!(encode_length(0).unwrap(), vec![0x00]);
        assert_eq!(encode_length(1).unwrap(), vec![0x01]);
        assert_eq!(encode_length(5).unwrap(), vec![0x05]);
        assert_eq!(encode_length(127).unwrap(), vec![0x7F]);
    }

    #[test]
    fn test_encode_length_long_form() {
        assert_eq!(encode_length(128).unwrap(), vec![0x81, 0x80]);
        assert_eq!(encode_length(255).unwrap(), vec![0x81, 0xFF]);
        assert_eq!(encode_length(256).unwrap(), vec![0x82, 0x01, 0x00]);
        assert_eq!(encode_length(1024).unwrap(), vec![0x82, 0x04, 0x00]);
        assert_eq!(encode_length(65535).unwrap(), vec![0x82, 0xFF, 0xFF]);
    }

    #[test]
    fn test_encode_boolean() {
        assert_eq!(encode_boolean("true").unwrap(), vec![0xFF]);
        assert_eq!(encode_boolean("TRUE").unwrap(), vec![0xFF]);
        assert_eq!(encode_boolean("false").unwrap(), vec![0x00]);
        assert_eq!(encode_boolean("FALSE").unwrap(), vec![0x00]);
        assert_eq!(encode_boolean("0xff").unwrap(), vec![0xFF]);
        assert_eq!(encode_boolean("0x00").unwrap(), vec![0x00]);
        assert!(encode_boolean("invalid").is_err());
    }

    #[test]
    fn test_encode_integer_positive() {
        assert_eq!(encode_integer("0").unwrap(), vec![0x00]);
        assert_eq!(encode_integer("1").unwrap(), vec![0x01]);
        assert_eq!(encode_integer("127").unwrap(), vec![0x7F]);
        assert_eq!(encode_integer("128").unwrap(), vec![0x00, 0x80]);
        assert_eq!(encode_integer("255").unwrap(), vec![0x00, 0xFF]);
        assert_eq!(encode_integer("256").unwrap(), vec![0x01, 0x00]);
        assert_eq!(encode_integer("32767").unwrap(), vec![0x7F, 0xFF]);
        assert_eq!(encode_integer("32768").unwrap(), vec![0x00, 0x80, 0x00]);
    }

    #[test]
    fn test_encode_integer_negative() {
        assert_eq!(encode_integer("-1").unwrap(), vec![0xFF]);
        assert_eq!(encode_integer("-128").unwrap(), vec![0x80]);
        assert_eq!(encode_integer("-129").unwrap(), vec![0xFF, 0x7F]);
        assert_eq!(encode_integer("-256").unwrap(), vec![0xFF, 0x00]);
    }

    #[test]
    fn test_encode_integer_hex() {
        assert_eq!(encode_integer("0x00").unwrap(), vec![0x00]);
        assert_eq!(encode_integer("0xFF").unwrap(), vec![0xFF]);
        assert_eq!(encode_integer("0x0102").unwrap(), vec![0x01, 0x02]);
    }

    #[test]
    fn test_encode_integer_invalid() {
        assert!(encode_integer("not_a_number").is_err());
        assert!(encode_integer("12.34").is_err());
    }

    #[test]
    fn test_encode_bit_string() {
        // Test with 0 unused bits
        assert_eq!(
            encode_bit_string("0 unused bits, data: 0102").unwrap(),
            vec![0x00, 0x01, 0x02]
        );
        
        // Test with 4 unused bits
        assert_eq!(
            encode_bit_string("4 unused bits, data: F0").unwrap(),
            vec![0x04, 0xF0]
        );
        
        // Test with longer data
        assert_eq!(
            encode_bit_string("0 unused bits, data: 010203040506").unwrap(),
            vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]
        );
    }

    #[test]
    fn test_encode_bit_string_invalid() {
        assert!(encode_bit_string("invalid format").is_err());
        assert!(encode_bit_string("data: FF").is_err()); // Missing unused bits
    }

    #[test]
    fn test_encode_octet_string() {
        assert_eq!(encode_octet_string("0102").unwrap(), vec![0x01, 0x02]);
        assert_eq!(encode_octet_string("FF").unwrap(), vec![0xFF]);
        assert_eq!(encode_octet_string("00").unwrap(), vec![0x00]);
        assert_eq!(encode_octet_string("010203040506070809").unwrap(), 
            vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    }

    #[test]
    fn test_encode_octet_string_with_spaces() {
        assert_eq!(encode_octet_string("01 02 03").unwrap(), vec![0x01, 0x02, 0x03]);
        assert_eq!(encode_octet_string("01:02:03").unwrap(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_encode_null() {
        assert_eq!(encode_null("").unwrap(), Vec::<u8>::new());
        assert_eq!(encode_null("anything").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_encode_oid_standard() {
        // OID 1.2.840.113549 encodes to 0x2A 0x86 0x48 0x86 0xF7 0x0D
        assert_eq!(
            encode_object_identifier("1.2.840.113549").unwrap(),
            vec![0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D]
        );
        
        // OID 2.5.4.3 (commonName)
        assert_eq!(
            encode_object_identifier("2.5.4.3").unwrap(),
            vec![0x55, 0x04, 0x03]
        );
    }

    #[test]
    fn test_encode_oid_with_large_numbers() {
        // Test with larger subidentifiers
        assert_eq!(
            encode_object_identifier("1.2.128").unwrap(),
            vec![0x2A, 0x81, 0x00]
        );
        
        assert_eq!(
            encode_object_identifier("1.2.16384").unwrap(),
            vec![0x2A, 0x81, 0x80, 0x00]
        );
    }

    #[test]
    fn test_encode_oid_invalid() {
        assert!(encode_object_identifier("1").is_err()); // Too short
        assert!(encode_object_identifier("invalid.oid").is_err());
        assert!(encode_object_identifier("1.2.abc").is_err());
    }

    #[test]
    fn test_encode_string() {
        assert_eq!(encode_string("test").unwrap(), vec![0x74, 0x65, 0x73, 0x74]);
        assert_eq!(encode_string("").unwrap(), Vec::<u8>::new());
        assert_eq!(encode_string("hello world").unwrap(), 
            b"hello world".to_vec());
    }

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(hex_to_bytes("0102").unwrap(), vec![0x01, 0x02]);
        assert_eq!(hex_to_bytes("FF").unwrap(), vec![0xFF]);
        assert_eq!(hex_to_bytes("00").unwrap(), vec![0x00]);
        assert_eq!(hex_to_bytes("01 02 03").unwrap(), vec![0x01, 0x02, 0x03]);
        assert_eq!(hex_to_bytes("01:02:03").unwrap(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_hex_to_bytes_invalid() {
        assert!(hex_to_bytes("0G").is_err());
        assert!(hex_to_bytes("XYZ").is_err());
    }

    #[test]
    fn test_encode_asn1_simple_integer() {
        let node = Asn1Node {
            label: "INTEGER".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 2,
            is_constructed: false,
            byte_offset: 0,
            byte_length: 0,
            length: 1,
            value: Some("42".to_string()),
            children: None,
        };
        
        let result = encode_asn1_tree(&node).unwrap();
        // 0x02 (INTEGER tag), 0x01 (length), 0x2A (42 in hex)
        assert_eq!(result, vec![0x02, 0x01, 0x2A]);
    }

    #[test]
    fn test_encode_asn1_sequence() {
        let child1 = Asn1Node {
            label: "INTEGER".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 2,
            is_constructed: false,
            byte_offset: 0,
            byte_length: 0,
            length: 1,
            value: Some("1".to_string()),
            children: None,
        };
        
        let child2 = Asn1Node {
            label: "INTEGER".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 2,
            is_constructed: false,
            byte_offset: 0,
            byte_length: 0,
            length: 1,
            value: Some("2".to_string()),
            children: None,
        };
        
        let sequence = Asn1Node {
            label: "SEQUENCE".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 16,
            is_constructed: true,
            byte_offset: 0,
            byte_length: 0,
            length: 6,
            value: None,
            children: Some(vec![child1, child2]),
        };
        
        let result = encode_asn1_tree(&sequence).unwrap();
        // 0x30 (SEQUENCE tag), 0x06 (length), 0x02 0x01 0x01, 0x02 0x01 0x02
        assert_eq!(result, vec![0x30, 0x06, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02]);
    }

    #[test]
    fn test_encode_asn1_boolean() {
        let node = Asn1Node {
            label: "BOOLEAN".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 1,
            is_constructed: false,
            byte_offset: 0,
            byte_length: 0,
            length: 1,
            value: Some("true".to_string()),
            children: None,
        };
        
        let result = encode_asn1_tree(&node).unwrap();
        // 0x01 (BOOLEAN tag), 0x01 (length), 0xFF (true)
        assert_eq!(result, vec![0x01, 0x01, 0xFF]);
    }

    #[test]
    fn test_encode_asn1_null() {
        let node = Asn1Node {
            label: "NULL".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 5,
            is_constructed: false,
            byte_offset: 0,
            byte_length: 0,
            length: 0,
            value: Some("".to_string()),
            children: None,
        };
        
        let result = encode_asn1_tree(&node).unwrap();
        // 0x05 (NULL tag), 0x00 (length)
        assert_eq!(result, vec![0x05, 0x00]);
    }

    #[test]
    fn test_encode_asn1_context_specific() {
        let node = Asn1Node {
            label: "[0]".to_string(),
            tag_class: "Context".to_string(),
            tag_number: 0,
            is_constructed: true,
            byte_offset: 0,
            byte_length: 0,
            length: 3,
            value: None,
            children: Some(vec![
                Asn1Node {
                    label: "INTEGER".to_string(),
                    tag_class: "Universal".to_string(),
                    tag_number: 2,
                    is_constructed: false,
                    byte_offset: 0,
                    byte_length: 0,
                    length: 1,
                    value: Some("5".to_string()),
                    children: None,
                }
            ]),
        };
        
        let result = encode_asn1_tree(&node).unwrap();
        // 0xA0 ([0] constructed), 0x03 (length), 0x02 0x01 0x05
        assert_eq!(result, vec![0xA0, 0x03, 0x02, 0x01, 0x05]);
    }

    #[test]
    fn test_encode_oid_component() {
        assert_eq!(encode_oid_component(0), vec![0x00]);
        assert_eq!(encode_oid_component(127), vec![0x7F]);
        assert_eq!(encode_oid_component(128), vec![0x81, 0x00]);
        assert_eq!(encode_oid_component(16383), vec![0xFF, 0x7F]);
        assert_eq!(encode_oid_component(16384), vec![0x81, 0x80, 0x00]);
    }

    #[test]
    fn test_encode_value_with_tag_number() {
        // Test UTF8String (tag 12)
        let result = encode_value(12, "test").unwrap();
        assert_eq!(result, b"test".to_vec());
        
        // Test PrintableString (tag 19)
        let result = encode_value(19, "hello").unwrap();
        assert_eq!(result, b"hello".to_vec());
        
        // Test unknown tag with hex
        let result = encode_value(99, "0x0102").unwrap();
        assert_eq!(result, vec![0x01, 0x02]);
    }

    #[test]
    fn test_round_trip_encoding() {
        // Create a simple structure and encode it
        let node = Asn1Node {
            label: "SEQUENCE".to_string(),
            tag_class: "Universal".to_string(),
            tag_number: 16,
            is_constructed: true,
            byte_offset: 0,
            byte_length: 0,
            length: 0,
            value: None,
            children: Some(vec![
                Asn1Node {
                    label: "INTEGER".to_string(),
                    tag_class: "Universal".to_string(),
                    tag_number: 2,
                    is_constructed: false,
                    byte_offset: 0,
                    byte_length: 0,
                    length: 1,
                    value: Some("100".to_string()),
                    children: None,
                },
                Asn1Node {
                    label: "OCTET STRING".to_string(),
                    tag_class: "Universal".to_string(),
                    tag_number: 4,
                    is_constructed: false,
                    byte_offset: 0,
                    byte_length: 0,
                    length: 3,
                    value: Some("010203".to_string()),
                    children: None,
                },
            ]),
        };
        
        let result = encode_asn1_tree(&node).unwrap();
        // Expected: SEQUENCE (0x30), length, INTEGER (0x02), length, value, OCTET STRING (0x04), length, value
        assert!(result.len() > 0);
        assert_eq!(result[0], 0x30); // SEQUENCE tag
        assert!(result.contains(&0x02)); // INTEGER tag
        assert!(result.contains(&0x04)); // OCTET STRING tag
    }
}
