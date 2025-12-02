// src/persona/membrane/signal_sanitizer.rs
use crate::persona::symbiosis::render_params::RenderParams;
use crate::constants::ETERNAL_SIGNATURE;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Helper để Serde xử lý mảng lớn > 32 phần tử
pub mod BigArray {
    use super::*;
    pub fn serialize<S: Serializer>(data: &[u8; 50], serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut s = serializer.serialize_tuple(50)?;
        for item in data {
            s.serialize_element(item)?;
        }
        s.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 50], D::Error> {
        let v: Vec<u8> = Vec::deserialize(deserializer)?;
        if v.len() == 50 {
            let mut array = [0u8; 50];
            array.copy_from_slice(&v);
            Ok(array)
        } else {
            Err(serde::de::Error::custom("Reserved field must be 50 bytes"))
        }
    }
}

impl RenderParams {
    /// Kiểm tra tính toàn vẹn của Holy Membrane.
    pub fn is_sanitized(&self) -> bool {
        // 1. Kiểm tra Chữ ký vĩnh cửu
        if self.eternal_signature != ETERNAL_SIGNATURE {
            println!("🛑 MEMBRANE ALERT: Invalid Signature {:?}", self.eternal_signature);
            return false;
        }
        true
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        unsafe { std::mem::transmute(*self) }
    }
    
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}
