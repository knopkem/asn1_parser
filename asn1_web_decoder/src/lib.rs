use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asn1Node {
    pub label: String,
    pub tag: u8,
    pub tag_class: String,
    pub is_constructed: bool,
    pub length: usize,
    pub value: Option<String>,
    pub children: Vec<Asn1Node>,
    pub byte_offset: usize,
    pub byte_length: usize,
}

#[wasm_bindgen]
pub fn decode_pem_to_json(pem_input: &str) -> Result<String, JsValue> {
    match decode_pem_internal(pem_input) {
        Ok(json) => Ok(json),
        Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
    }
}

#[wasm_bindgen]
pub fn pem_to_hex(pem_input: &str) -> Result<String, JsValue> {
    match pem::parse(pem_input) {
        Ok(pem) => {
            let hex_string = pem.contents()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            Ok(hex_string)
        }
        Err(e) => Err(JsValue::from_str(&format!("Error parsing PEM: {}", e))),
    }
}

fn decode_pem_internal(pem_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pem = pem::parse(pem_input)?;
    
    let mut root = Asn1Node {
        label: format!("PEM: {}", pem.tag()),
        tag: 0,
        tag_class: "PEM".to_string(),
        is_constructed: true,
        length: pem.contents().len(),
        value: None,
        children: Vec::new(),
        byte_offset: 0,
        byte_length: pem.contents().len(),
    };
    
    decode_der_recursive(pem.contents(), &mut root.children, 0)?;
    
    Ok(serde_json::to_string_pretty(&root)?)
}

fn decode_der_recursive(data: &[u8], nodes: &mut Vec<Asn1Node>, base_offset: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut pos = 0;
    
    while pos < data.len() {
        if pos >= data.len() {
            break;
        }
        
        let node_start = pos;
        let tag = data[pos];
        pos += 1;
        
        if pos >= data.len() {
            break;
        }
        
        let (length, length_bytes) = parse_length(&data[pos..])?;
        pos += length_bytes;
        
        if pos + length > data.len() {
            break;
        }
        
        let tag_class = (tag & 0xC0) >> 6;
        let is_constructed = (tag & 0x20) != 0;
        let tag_number = tag & 0x1F;
        
        let tag_class_str = match tag_class {
            0 => "UNIVERSAL",
            1 => "APPLICATION",
            2 => "CONTEXT",
            3 => "PRIVATE",
            _ => "UNKNOWN",
        };
        
        let tag_type = get_universal_tag_name(tag_number);
        
        let label = if tag_class == 0 {
            format!("{} (Tag {})", tag_type, tag_number)
        } else {
            format!("[{}] Tag {}", tag_class_str, tag_number)
        };
        
        let content = &data[pos..pos + length];
        let total_length = 1 + length_bytes + length; // tag + length bytes + content
        
        let mut node = Asn1Node {
            label,
            tag: tag_number,
            tag_class: tag_class_str.to_string(),
            is_constructed,
            length,
            value: None,
            children: Vec::new(),
            byte_offset: base_offset + node_start,
            byte_length: total_length,
        };
        
        if is_constructed {
            decode_der_recursive(content, &mut node.children, base_offset + pos)?;
        } else {
            node.value = Some(decode_value(tag_number, content));
        }
        
        // For BIT STRING, also show value even if constructed
        if tag_number == 3 && node.value.is_none() {
            node.value = Some(decode_bit_string(content));
        }
        
        nodes.push(node);
        pos += length;
    }
    
    Ok(())
}

fn parse_length(data: &[u8]) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Err("Unexpected end of data".into());
    }
    
    let first_byte = data[0];
    
    if first_byte & 0x80 == 0 {
        Ok((first_byte as usize, 1))
    } else {
        let num_octets = (first_byte & 0x7F) as usize;
        if num_octets == 0 || num_octets > 4 {
            return Err("Invalid length encoding".into());
        }
        
        if data.len() < 1 + num_octets {
            return Err("Unexpected end of data".into());
        }
        
        let mut length = 0usize;
        for i in 0..num_octets {
            length = (length << 8) | (data[1 + i] as usize);
        }
        
        Ok((length, 1 + num_octets))
    }
}

fn get_universal_tag_name(tag: u8) -> &'static str {
    match tag {
        1 => "BOOLEAN",
        2 => "INTEGER",
        3 => "BIT STRING",
        4 => "OCTET STRING",
        5 => "NULL",
        6 => "OBJECT IDENTIFIER",
        12 => "UTF8String",
        16 => "SEQUENCE",
        17 => "SET",
        19 => "PrintableString",
        20 => "TeletexString",
        22 => "IA5String",
        23 => "UTCTime",
        24 => "GeneralizedTime",
        _ => "Unknown",
    }
}

fn decode_value(tag: u8, content: &[u8]) -> String {
    match tag {
        2 => decode_integer(content),
        3 => decode_bit_string(content),
        4 => decode_octet_string(content),
        6 => format!("OID: {}", decode_oid(content)),
        12 | 19 | 20 | 22 => {
            if let Ok(s) = std::str::from_utf8(content) {
                format!("\"{}\"", s)
            } else {
                hex_string(content)
            }
        }
        23 | 24 => {
            if let Ok(s) = std::str::from_utf8(content) {
                format!("Time: {}", s)
            } else {
                hex_string(content)
            }
        }
        1 => format!("{}", content.get(0).map_or(false, |&b| b != 0)),
        5 => "NULL".to_string(),
        _ => {
            if content.len() <= 32 {
                hex_string(content)
            } else {
                format!("{} bytes", content.len())
            }
        }
    }
}

fn decode_integer(data: &[u8]) -> String {
    if data.is_empty() {
        return "0".to_string();
    }
    
    if data.len() <= 8 {
        let mut value: i64 = if data[0] & 0x80 != 0 { -1 } else { 0 };
        for &byte in data {
            value = (value << 8) | (byte as i64);
        }
        format!("{}", value)
    } else {
        format!("0x{}", hex_string(data))
    }
}

fn decode_bit_string(data: &[u8]) -> String {
    if data.is_empty() {
        return "Empty".to_string();
    }
    let unused_bits = data[0];
    // Always show the hex data for BIT STRINGs
    format!("{} unused bits, data: {}", unused_bits, hex_string(&data[1..]))
}

fn decode_octet_string(data: &[u8]) -> String {
    if data.len() <= 32 {
        hex_string(data)
    } else {
        format!("{} bytes", data.len())
    }
}

fn decode_oid(data: &[u8]) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    
    let mut result = Vec::new();
    let first = data[0];
    result.push(first / 40);
    result.push(first % 40);
    
    let mut value = 0u64;
    for &byte in &data[1..] {
        value = (value << 7) | ((byte & 0x7F) as u64);
        if byte & 0x80 == 0 {
            result.push(value as u8);
            value = 0;
        }
    }
    
    result.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(".")
}

fn hex_string(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("")
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_string_value_primitive() {
        // Test a primitive BIT STRING
        // DER encoding: 03 04 06 6e 5d c0
        // Tag: 03 (BIT STRING)
        // Length: 04
        // Content: 06 6e 5d c0 (06 = unused bits, rest is data)
        let pem_str = "-----BEGIN TEST-----\nAwQGbl3A\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        
        assert!(result.is_ok(), "Failed to decode PEM: {:?}", result.err());
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        // Navigate to the BIT STRING node
        let bit_string = &parsed["children"][0];
        assert_eq!(bit_string["label"].as_str().unwrap(), "BIT STRING (Tag 3)");
        
        // Check that value is present
        let value = bit_string["value"].as_str();
        assert!(value.is_some(), "BIT STRING value should be present");
        assert!(value.unwrap().contains("unused bits"), "Value should mention unused bits");
    }

    #[test]
    fn test_bit_string_value_constructed() {
        // Test a constructed BIT STRING (which might contain nested structures)
        // Tag: 23 (0x03 with constructed bit set = 0x23)
        // DER: 23 08 03 03 00 ab cd 03 02 00 ef
        // Base64: IwgDAwCrzQMCAO8=
        let pem_str = "-----BEGIN TEST-----\nIwgDAwCrzQMCAO8=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        
        assert!(result.is_ok(), "Failed to decode PEM: {:?}", result.err());
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        // Navigate to the constructed BIT STRING node
        let bit_string = &parsed["children"][0];
        
        // Check that value is present even for constructed BIT STRING
        let value = bit_string["value"].as_str();
        assert!(value.is_some(), "Constructed BIT STRING should also have a value");
        println!("Constructed BIT STRING value: {:?}", value);
    }

    #[test]
    fn test_decode_bit_string_function() {
        // Test the decode_bit_string function directly
        let data = vec![0x00, 0x30, 0x82, 0x01, 0x22];  // 0 unused bits, then data
        let result = decode_bit_string(&data);
        
        assert!(result.contains("0 unused bits"));
        assert!(result.to_lowercase().contains("30820122"));
    }
}
