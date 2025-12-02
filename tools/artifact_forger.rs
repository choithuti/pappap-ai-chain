// tools/artifact_forger.rs
// Compile & Run: cargo run --bin artifact_forger
use std::fs::{self, File};
use std::io::Write;

// Kích thước bắt buộc
const GENESIS_SIZE: usize = 4089;
const AIR_GAP_SIZE: usize = 8185;
const ETERNAL_SIGNATURE: [u8; 7] = [7, 7, 7, 7, 7, 7, 7];

fn main() {
    println!("🛠️  ARTIFACT FORGER: INITIATING...");

    // 1. Tạo thư mục đích
    let _ = fs::create_dir_all("core/bootstrap");
    let _ = fs::create_dir_all("persona/membrane");

    // 2. Rèn Genesis Reader (4089 bytes)
    // Giả lập header WASM
    let mut genesis_data = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; 
    // Padding cho đủ kích thước
    while genesis_data.len() < GENESIS_SIZE {
        genesis_data.push(0x00); // Lấp đầy bằng Void
    }
    
    let mut f1 = File::create("core/bootstrap/genesis_reader.wasm").expect("Cannot create Genesis file");
    f1.write_all(&genesis_data).expect("Write failed");
    println!("✅ Forged: core/bootstrap/genesis_reader.wasm ({} bytes)", genesis_data.len());

    // 3. Rèn Air Gap (8185 bytes)
    let mut air_gap_data = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    
    // Padding cho đến vị trí Signature
    // Ta cần để dành 7 byte cuối cùng cho Signature
    while air_gap_data.len() < AIR_GAP_SIZE - ETERNAL_SIGNATURE.len() {
        air_gap_data.push(0xFF); // Lấp đầy bằng Firewall (0xFF)
    }
    
    // Đóng dấu Eternal Signature
    air_gap_data.extend_from_slice(&ETERNAL_SIGNATURE);
    
    let mut f2 = File::create("persona/membrane/air_gap.wasm").expect("Cannot create AirGap file");
    f2.write_all(&air_gap_data).expect("Write failed");
    
    // Kiểm tra lại kích thước
    if air_gap_data.len() != AIR_GAP_SIZE {
        panic!("❌ FORGING ERROR: Air Gap size mismatch! Got {}, Expected {}", air_gap_data.len(), AIR_GAP_SIZE);
    }

    println!("✅ Forged: persona/membrane/air_gap.wasm ({} bytes)", air_gap_data.len());
    println!("🔒 ETERNAL SIGNATURE SEALED.");
}
