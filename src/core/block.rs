use crate::constants::*;
use crate::core::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub prev_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub spike_score: f32,
    pub miner: String,
    pub eternal_signature: [u8; 7],
    pub forbidden_gene_check: bool,
}

impl Block {
    pub fn new(index: u64, prev_hash: String, txs: Vec<Transaction>, miner: String, spike: f32) -> Self {
        let mut b = Self {
            index,
            timestamp: chrono::Utc::now().timestamp(),
            prev_hash,
            hash: String::new(),
            transactions: txs,
            spike_score: spike,
            miner,
            eternal_signature: ETERNAL_SIGNATURE,
            forbidden_gene_check: false,
        };
        // Kiểm tra Gene cấm trước khi tính hash
        if FORBIDDEN_GENES.contains(&index) {
            panic!("❌ BLOCK REJECTED: Contains Forbidden Gene {}", index);
        }
        b.forbidden_gene_check = true;
        b.hash = b.calculate_hash();
        b
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}{}{:?}",
            self.index, self.timestamp, self.prev_hash, self.spike_score,
            self.miner, self.forbidden_gene_check, self.eternal_signature
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }
}
