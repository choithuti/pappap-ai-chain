
// src/persona/symbiosis/render_params.rs
use serde::{Serialize, Deserialize};
use crate::constants::ETERNAL_SIGNATURE;

/// RenderParams: Output duy nhất của linh hồn (64 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RenderParams {
    pub hue: u8,
    pub saturation: u8,
    pub brightness: u8,
    pub curvature: u8,
    pub tempo: u8,
    pub warmth: u8,
    pub depth: u8,

    // Sử dụng module BigArray từ signal_sanitizer để serialize mảng 50 phần tử
    #[serde(with = "crate::persona::membrane::signal_sanitizer::BigArray")]
    pub reserved: [u8; 50],

    pub eternal_signature: [u8; 7],
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            hue: 0,
            saturation: 0,
            brightness: 0,
            curvature: 0,
            tempo: 60,
            warmth: 127,
            depth: 0,
            reserved: [0; 50],
            eternal_signature: ETERNAL_SIGNATURE,
        }
    }
}
