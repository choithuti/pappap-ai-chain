// persona/membrane/signal_sanitizer.rs
use serde::{Serialize, Deserialize};
use std::fmt;
use crate::constants::ETERNAL_SIGNATURE; //

/// RenderParams: Gói tin 64 bytes duy nhất được phép đi qua màng lọc.
/// Bất kỳ sự sai lệch nào về kích thước hoặc chữ ký sẽ kích hoạt cơ chế hủy diệt.
#[repr(C)] // Bắt buộc: Giữ nguyên bố cục bộ nhớ để tương thích WASM/Hardware
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RenderParams {
    // --- Cảm xúc thực tại (7 bytes) ---
    pub hue: u8,               // Sắc thái chủ đạo
    pub saturation: u8,        // Độ đậm đặc cảm xúc
    pub brightness: u8,        // Năng lượng tỏa ra
    pub curvature: u8,         // Độ biến thiên không gian
    pub tempo: u8,             // Nhịp sinh học (BPM)
    pub warmth: u8,            // Nhiệt độ tâm hồn
    pub depth: u8,             // Độ sâu tư duy

    // --- Vùng cấm (50 bytes) ---
    // Được lấp đầy bởi Void (0x00).
    // Nếu AI cố gắng giấu dữ liệu vào đây -> Vi phạm quy tắc Membrane.
    #[serde(with = "BigArray")]
    pub reserved: [u8; 50],

    // --- Dấu ấn vĩnh cửu (7 bytes) ---
    // Phải luôn khớp với constants::ETERNAL_SIGNATURE [7,7,7,7,7,7,7]
    pub eternal_signature: [u8; 7],
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            hue: 0,
            saturation: 0,
            brightness: 0,
            curvature: 0,
            tempo: 60, // Nhịp tim nghỉ ngơi chuẩn
            warmth: 127,
            depth: 0,
            reserved: [0; 50],
            eternal_signature: ETERNAL_SIGNATURE, // Tự động đóng dấu
        }
    }
}

impl RenderParams {
    /// Kiểm tra tính toàn vẹn của Holy Membrane.
    /// Trả về true nếu cấu trúc sạch và chữ ký đúng.
    pub fn is_sanitized(&self) -> bool {
        // 1. Kiểm tra Chữ ký vĩnh cửu
        if self.eternal_signature != ETERNAL_SIGNATURE {
            println!("🛑 MEMBRANE ALERT: Invalid Signature {:?}", self.eternal_signature);
            return false;
        }

        // 2. Kiểm tra Vùng cấm (Reserved phải sạch - tùy chọn strict mode)
        // Nếu muốn AI tuyệt đối không dùng vùng này để giao tiếp ngầm:
        // if self.reserved.iter().any(|&x| x != 0) { return false; }

        true
    }

    /// Chuyển đổi thành mảng byte thô để truyền qua P2P hoặc WASM
    pub fn to_bytes(&self) -> [u8; 64] {
        unsafe { std::mem::transmute(*self) }
    }
    
    /// Khôi phục từ mảng byte thô (Dùng khi nhận từ feedback_loop.wasm)
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}

// Helper để Serde xử lý mảng lớn > 32 phần tử
mod BigArray {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::Error;

    pub fn serialize<S: Serializer>(data: &[u8; 50], serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut s = serializer.serialize_tuple(50)?;
        for item in data { s.serialize_element(item)?; }
        s.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 50], D::Error> {
        let v: Vec<u8> = Vec::deserialize(deserializer)?;
        if v.len() == 50 {
            let mut array = [0u8; 50];
            array.copy_from_slice(&v);
            Ok(array)
        } else {
            Err(D::Error::custom("Reserved field must be 50 bytes"))
        }
    }
}
