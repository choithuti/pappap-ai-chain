// persona/symbiosis/render_params.rs
use serde::{Serialize, Deserialize};

#[repr(C)] // Quan trọng: Giữ đúng thứ tự bộ nhớ
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RenderParams {
    // --- Cảm xúc (7 bytes) ---
    pub hue: u8,
    pub saturation: u8,
    pub brightness: u8,
    pub curvature: u8,
    pub tempo: u8,
    pub warmth: u8,
    pub depth: u8,

    // --- Khoảng lặng (50 bytes) ---
    // Dành cho sự mở rộng trong tương lai, hiện tại là Void
    pub reserved: [u8; 50],

    // --- Dấu ấn vĩnh cửu (7 bytes) ---
    // Luôn luôn là [7, 7, 7, 7, 7, 7, 7]
    pub eternal_signature: [u8; 7],
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            hue: 0, saturation: 0, brightness: 0, 
            curvature: 0, tempo: 60, warmth: 127, depth: 0,
            reserved: [0; 50],
            eternal_signature: [7; 7],
        }
    }
}
