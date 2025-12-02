// src/core/block.rs
use crate::constants::{ETERNAL_SIGNATURE, FORBIDDEN_GENES};
use crate::core::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub spike_score: f32,       // Điểm số trí tuệ từ AI (Proof of Intelligence)
    pub miner: String,
    pub eternal_signature: [u8; 7],
    pub forbidden_gene_checked: bool,
}

impl Block {
    pub fn new(
        index: u64,
        prev_hash: String,
        transactions: Vec<Transaction>,
        miner: String,
        spike_score: f32, // Thay vì DNum, ta dùng f32 ở lớp giao tiếp để đơn giản hóa serialize
    ) -> Self {
        let mut block = Self {
            index,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            prev_hash,
            hash: String::new(),
            transactions,
            spike_score,
            miner,
            eternal_signature: ETERNAL_SIGNATURE,
            forbidden_gene_checked: false,
        };

        // BẮT BUỘC: Kiểm tra Gene cấm
        if FORBIDDEN_GENES.contains(&index) {
            panic!("🚫 BLOCK REJECTED: Forbidden Gene {} Detected.", index);
        }
        block.forbidden_gene_checked = true;
        
        // Tính toán Hash sau khi đã điền đầy đủ thông tin
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        // Gom tất cả dữ liệu thành chuỗi để hash
        let tx_data = self.transactions.iter()
            .map(|tx| tx.id.clone())
            .collect::<String>();
            
        let input = format!(
            "{}{}{}{}{}{}{:?}{}",
            self.index,
            self.timestamp,
            self.prev_hash,
            self.spike_score,
            self.miner,
            self.forbidden_gene_checked,
            self.eternal_signature,
            tx_data
        );

        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }
}
