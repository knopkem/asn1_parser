use crate::Asn1Node;

pub fn decode_pem_internal(pem_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pem = pem::parse(pem_input)?;
    
    let mut root = Asn1Node {
        label: format!("PEM: {}", pem.tag()),
        tag: 0,
        tag_number: 0,
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

pub fn decode_der_recursive(data: &[u8], nodes: &mut Vec<Asn1Node>, base_offset: usize) -> Result<(), Box<dyn std::error::Error>> {
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
            tag_number: tag_number as u32,
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
        
        let mut length: usize = 0;
        for i in 0..num_octets {
            length = (length << 8) | (data[1 + i] as usize);
        }
        
        Ok((length, 1 + num_octets))
    }
}

fn decode_value(tag: u8, content: &[u8]) -> String {
    match tag {
        1 => decode_boolean(content),
        2 => decode_integer(content),
        3 => decode_bit_string(content),
        4 => decode_octet_string(content),
        5 => "NULL".to_string(),
        6 => decode_oid(content),
        12 => decode_utf8_string(content),
        13 => decode_relative_oid(content),
        19 => decode_printable_string(content),
        22 => decode_ia5_string(content),
        23 => decode_utc_time(content),
        24 => decode_generalized_time(content),
        _ => format!("[{} bytes]", content.len()),
    }
}

fn decode_boolean(data: &[u8]) -> String {
    if data.is_empty() {
        return "Invalid BOOLEAN".to_string();
    }
    if data[0] == 0 {
        "FALSE".to_string()
    } else {
        "TRUE".to_string()
    }
}

fn decode_integer(data: &[u8]) -> String {
    if data.is_empty() {
        return "0".to_string();
    }
    
    // Check if negative (first bit is 1)
    let is_negative = (data[0] & 0x80) != 0;
    
    if data.len() <= 8 {
        let mut value: i64 = if is_negative { -1 } else { 0 };
        for &byte in data {
            value = (value << 8) | (byte as i64);
        }
        value.to_string()
    } else {
        // For large integers, show hex
        format!("0x{}", data.iter().map(|b| format!("{:02X}", b)).collect::<String>())
    }
}

fn decode_bit_string(data: &[u8]) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    
    let unused_bits = data[0];
    let content = &data[1..];
    
    if content.is_empty() {
        return format!("(unused bits: {})", unused_bits);
    }
    
    let mut bits = String::new();
    for (i, &byte) in content.iter().enumerate() {
        for bit_pos in (0..8).rev() {
            // Skip unused bits in the last byte
            if i == content.len() - 1 && bit_pos < unused_bits {
                continue;
            }
            bits.push(if (byte & (1 << bit_pos)) != 0 { '1' } else { '0' });
        }
    }
    
    format!("{} (unused bits: {})", bits, unused_bits)
}

fn decode_octet_string(data: &[u8]) -> String {
    // Try to decode as UTF-8 string first
    if let Ok(s) = std::str::from_utf8(data) {
        if s.chars().all(|c| !c.is_control() || c == '\n' || c == '\r' || c == '\t') {
            return format!("\"{}\"", s);
        }
    }
    
    // Otherwise show as hex
    format!("[{} bytes] {}", data.len(), 
            data.iter().take(16).map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
}

fn decode_utf8_string(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => format!("\"{}\"", s),
        Err(_) => format!("[Invalid UTF-8: {} bytes]", data.len()),
    }
}

fn decode_printable_string(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => format!("\"{}\"", s),
        Err(_) => format!("[Invalid PrintableString: {} bytes]", data.len()),
    }
}

fn decode_ia5_string(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => format!("\"{}\"", s),
        Err(_) => format!("[Invalid IA5String: {} bytes]", data.len()),
    }
}

fn decode_utc_time(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => format!("\"{}\"", s),
        Err(_) => format!("[Invalid UTCTime: {} bytes]", data.len()),
    }
}

fn decode_generalized_time(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => format!("\"{}\"", s),
        Err(_) => format!("[Invalid GeneralizedTime: {} bytes]", data.len()),
    }
}

fn decode_oid(data: &[u8]) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    
    let first = data[0];
    let first_component = first / 40;
    let second_component = first % 40;
    
    let mut oid = format!("{}.{}", first_component, second_component);
    
    let mut pos = 1;
    while pos < data.len() {
        let mut value: u64 = 0;
        loop {
            if pos >= data.len() {
                break;
            }
            let byte = data[pos];
            pos += 1;
            value = (value << 7) | ((byte & 0x7F) as u64);
            if (byte & 0x80) == 0 {
                break;
            }
        }
        oid.push_str(&format!(".{}", value));
    }
    
    oid
}

fn decode_relative_oid(data: &[u8]) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    
    let mut oid = String::new();
    let mut pos = 0;
    let mut first = true;
    
    while pos < data.len() {
        let mut value: u64 = 0;
        loop {
            if pos >= data.len() {
                break;
            }
            let byte = data[pos];
            pos += 1;
            value = (value << 7) | ((byte & 0x7F) as u64);
            if (byte & 0x80) == 0 {
                break;
            }
        }
        if first {
            oid.push_str(&format!("{}", value));
            first = false;
        } else {
            oid.push_str(&format!(".{}", value));
        }
    }
    
    oid
}

fn get_universal_tag_name(tag: u8) -> &'static str {
    match tag {
        1 => "BOOLEAN",
        2 => "INTEGER",
        3 => "BIT STRING",
        4 => "OCTET STRING",
        5 => "NULL",
        6 => "OBJECT IDENTIFIER",
        7 => "ObjectDescriptor",
        8 => "EXTERNAL",
        9 => "REAL",
        10 => "ENUMERATED",
        11 => "EMBEDDED PDV",
        12 => "UTF8String",
        13 => "RELATIVE-OID",
        16 => "SEQUENCE",
        17 => "SET",
        18 => "NumericString",
        19 => "PrintableString",
        20 => "TeletexString",
        21 => "VideotexString",
        22 => "IA5String",
        23 => "UTCTime",
        24 => "GeneralizedTime",
        25 => "GraphicString",
        26 => "VisibleString",
        27 => "GeneralString",
        28 => "UniversalString",
        29 => "CHARACTER STRING",
        30 => "BMPString",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer_small() {
        let data = vec![0x01];
        let result = decode_integer(&data);
        assert_eq!(result, "1");
    }

    #[test]
    fn test_decode_integer_negative() {
        let data = vec![0xFF];
        let result = decode_integer(&data);
        assert_eq!(result, "-1");
    }

    #[test]
    fn test_decode_integer_large() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let result = decode_integer(&data);
        assert!(result.starts_with("0x"));
    }

    #[test]
    fn test_decode_boolean_false() {
        let data = vec![0x00];
        assert_eq!(decode_boolean(&data), "FALSE");
    }

    #[test]
    fn test_decode_boolean_true() {
        let data = vec![0xFF];
        assert_eq!(decode_boolean(&data), "TRUE");
    }

    #[test]
    fn test_decode_oid() {
        // OID 1.2.840.113549 (RSA)
        let data = vec![0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D];
        let result = decode_oid(&data);
        assert_eq!(result, "1.2.840.113549");
    }

    #[test]
    fn test_decode_bit_string() {
        // Bit string with 1 unused bit: 01010101 0
        let data = vec![0x01, 0xAA];
        let result = decode_bit_string(&data);
        assert!(result.contains("1010101"));
        assert!(result.contains("unused bits: 1"));
    }

    #[test]
    fn test_decode_utf8_string() {
        let data = b"Hello, World!";
        let result = decode_utf8_string(data);
        assert_eq!(result, "\"Hello, World!\"");
    }

    #[test]
    fn test_parse_length_short() {
        let data = vec![0x05];
        let (length, bytes_used) = parse_length(&data).unwrap();
        assert_eq!(length, 5);
        assert_eq!(bytes_used, 1);
    }

    #[test]
    fn test_parse_length_long() {
        let data = vec![0x81, 0xFF];
        let (length, bytes_used) = parse_length(&data).unwrap();
        assert_eq!(length, 255);
        assert_eq!(bytes_used, 2);
    }

    #[test]
    fn test_get_universal_tag_name() {
        assert_eq!(get_universal_tag_name(2), "INTEGER");
        assert_eq!(get_universal_tag_name(16), "SEQUENCE");
        assert_eq!(get_universal_tag_name(6), "OBJECT IDENTIFIER");
    }

    #[test]
    fn test_decode_sequence() {
        // SEQUENCE { INTEGER 1 }
        // 30 03 02 01 01
        let pem_str = "-----BEGIN TEST-----\nMAMCAQE=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let sequence = &parsed["children"][0];
        assert!(sequence["label"].as_str().unwrap().contains("SEQUENCE"));
        assert_eq!(sequence["tag_number"].as_u64().unwrap(), 16);
        assert_eq!(sequence["is_constructed"].as_bool().unwrap(), true);
        
        let integer = &sequence["children"][0];
        assert!(integer["label"].as_str().unwrap().contains("INTEGER"));
        assert_eq!(integer["value"].as_str().unwrap(), "1");
    }

    #[test]
    fn test_decode_oid_in_sequence() {
        // SEQUENCE { OID 1.2.840.113549 }
        // 30 08 06 06 2A 86 48 86 F7 0D
        let pem_str = "-----BEGIN TEST-----\nMAgGBiqGSIb3DQ==\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let oid = &parsed["children"][0]["children"][0];
        assert!(oid["label"].as_str().unwrap().contains("OBJECT IDENTIFIER"));
        assert_eq!(oid["value"].as_str().unwrap(), "1.2.840.113549");
    }

    #[test]
    fn test_decode_nested_sequence() {
        // SEQUENCE { SEQUENCE { INTEGER 1 } }
        // 30 05 30 03 02 01 01
        let pem_str = "-----BEGIN TEST-----\nMAUwAwIBAQ==\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let outer = &parsed["children"][0];
        assert!(outer["is_constructed"].as_bool().unwrap());
        
        let inner = &outer["children"][0];
        assert!(inner["is_constructed"].as_bool().unwrap());
        assert_eq!(inner["children"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_decode_context_specific_tag() {
        // [0] IMPLICIT INTEGER 5
        // A0 03 02 01 05
        let pem_str = "-----BEGIN TEST-----\noAMCAQU=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let node = &parsed["children"][0];
        assert_eq!(node["tag_class"].as_str().unwrap(), "CONTEXT");
        assert_eq!(node["tag_number"].as_u64().unwrap(), 0);
        assert_eq!(node["is_constructed"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_decode_application_tag() {
        // APPLICATION 5 (constructed) { INTEGER 1 }
        // 65 03 02 01 01
        let pem_str = "-----BEGIN TEST-----\nZQMCAQE=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let node = &parsed["children"][0];
        assert_eq!(node["tag_class"].as_str().unwrap(), "APPLICATION");
        assert_eq!(node["tag_number"].as_u64().unwrap(), 5);
    }

    #[test]
    fn test_decode_private_tag() {
        // PRIVATE 10 (constructed) { INTEGER 1 }
        // EA 03 02 01 01
        let pem_str = "-----BEGIN TEST-----\n6gMCAQE=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let node = &parsed["children"][0];
        assert_eq!(node["tag_class"].as_str().unwrap(), "PRIVATE");
        assert_eq!(node["tag_number"].as_u64().unwrap(), 10);
    }

    #[test]
    fn test_byte_offsets() {
        // SEQUENCE { INTEGER 1 }
        // 30 03 02 01 01
        let pem_str = "-----BEGIN TEST-----\nMAMCAQE=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let sequence = &parsed["children"][0];
        assert_eq!(sequence["byte_offset"].as_u64().unwrap(), 0);
        assert_eq!(sequence["byte_length"].as_u64().unwrap(), 5);
        
        let integer = &sequence["children"][0];
        assert_eq!(integer["byte_offset"].as_u64().unwrap(), 2);
        assert_eq!(integer["byte_length"].as_u64().unwrap(), 3);
    }

    #[test]
    fn test_decode_pem_internal() {
        // Test the internal decode function directly (not the wasm-bindgen wrapper)
        let pem_str = "-----BEGIN TEST-----\nMAMCAQE=\n-----END TEST-----";
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json_str = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed.is_object());
        
        // The root is a PEM container, check it has children
        assert!(parsed["children"].is_array());
        let children = parsed["children"].as_array().unwrap();
        assert!(children.len() > 0, "Should have at least one child node");
        
        // The first child should be a SEQUENCE containing an INTEGER
        let first_child = &children[0];
        assert!(first_child["label"].as_str().unwrap().contains("SEQUENCE"));
    }
    
    #[test]
    fn test_decode_pem_internal_error() {
        let result = decode_pem_internal("invalid pem");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_set() {
        // SET { INTEGER 1, INTEGER 2 }
        // 31 06 02 01 01 02 01 02
        let pem_str = "-----BEGIN TEST-----\nMQYCAQECAQI=\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let set = &parsed["children"][0];
        assert!(set["label"].as_str().unwrap().contains("SET"));
        assert_eq!(set["tag_number"].as_u64().unwrap(), 17);
    }

    #[test]
    fn test_malformed_der_truncated() {
        // Incomplete DER structure
        let pem_str = "-----BEGIN TEST-----\nMAM=\n-----END TEST-----"; // Only 3 bytes
        let result = decode_pem_internal(pem_str);
        // Should not panic, may succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_long_length_encoding() {
        // Create a SEQUENCE with length encoded in long form
        // 30 81 03 02 01 01 (length = 3 in long form: 81 03)
        let pem_str = "-----BEGIN TEST-----\nMIEDAgEB\n-----END TEST-----";
        
        let result = decode_pem_internal(pem_str);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        let sequence = &parsed["children"][0];
        assert_eq!(sequence["length"].as_u64().unwrap(), 3);
    }
}
