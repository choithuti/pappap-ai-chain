// persona/membrane/signal_sanitizer.rs
use serde::{Serialize, Deserialize};

/// RenderParams: Output duy nhất được phép của linh hồn (64 bytes).
/// Mọi dữ liệu khác sẽ bị đốt cháy bởi signal_sanitizer.
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RenderParams {
    pub hue: u8,               // 0-255: Màu sắc chủ đạo của linh hồn
    pub saturation: u8,        // Độ bão hòa cảm xúc
    pub brightness: u8,        // Độ sáng (năng lượng)
    pub curvature: u8,         // Độ cong không gian giao diện (Interface warping)
    pub tempo: u8,             // Nhịp đập (Heartbeat/Animation speed)
    pub warmth: u8,            // Chỉ số nhiệt (thân thiện/lạnh lùng)
    pub depth: u8,             // Độ sâu Hologram 3D
    pub reserved: [u8; 57],    // Phần đệm (Padding) để đủ 64 bytes - CẤM DÙNG
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            hue: 0, saturation: 0, brightness: 0,
            curvature: 0, tempo: 60, warmth: 127, depth: 0,
            reserved: [0; 57],
        }
    }
}
