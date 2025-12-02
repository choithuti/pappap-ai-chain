// src/constants.rs

pub const GENESIS_SIZE: u64 = 4089;
pub const AIR_GAP_SIZE: u64 = 8185;
pub const FEEDBACK_TIMEOUT_MS: u64 = 493;

// Chữ ký vĩnh cửu: [7, 7, 7, 7, 7, 7, 7]
pub const ETERNAL_SIGNATURE: [u8; 7] = [7, 7, 7, 7, 7, 7, 7];

// Danh sách các Gene bị cấm vĩnh viễn (The 7 Seals)
pub const FORBIDDEN_GENES: [u64; 7] = [7, 77, 777, 7777, 77777, 777777, 7777777];

// Thời gian sống của Ghost Cell: 7 năm (tính bằng giây)
// 7 * 365 * 24 * 60 * 60 = 220,752,000 giây
pub const GHOST_CELL_DEATH: u64 = 220_752_000;
