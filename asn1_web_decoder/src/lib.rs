use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod encoder;
mod decoder;

use encoder::{encode_asn1_tree, Asn1Node as EncoderNode};
use decoder::decode_pem_internal;

#[derive(Serialize, Deserialize, Debug)]
pub struct Asn1Node {
    pub label: String,
    pub tag: u8,
    pub tag_class: String,
    pub tag_number: u32,
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
                .map(|b| format!("{:02X}", b))
                .collect::<String>();
            Ok(hex_string)
        }
        Err(e) => Err(JsValue::from_str(&format!("Error parsing PEM: {}", e))),
    }
}

#[wasm_bindgen]
pub fn encode_asn1_to_pem(json_str: &str, label: &str) -> Result<String, JsValue> {
    let root: EncoderNode = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;
    
    let der_bytes = encode_asn1_tree(&root)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode ASN.1: {}", e)))?;
    
    let pem = pem::Pem::new(label, der_bytes);
    Ok(pem::encode(&pem))
}
