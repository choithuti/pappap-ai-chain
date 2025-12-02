// src/core/storage.rs
use sled::{Db, IVec};
use std::str::from_utf8;
use serde::{Serialize, Deserialize};
use crate::core::block::Block; // Giả sử đã có struct Block

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        println!("💾 MOUNTING SLED DB AT: {}", path);
        let db = sled::open(path).expect("Failed to open database");
        Self { db }
    }

    // --- Block Persistence ---
    pub fn save_block(&self, block: &Block) {
        let key = format!("block:{}", block.index);
        let value = serde_json::to_vec(block).unwrap();
        self.db.insert(key, value).unwrap();
        
        // Lưu chiều cao block mới nhất
        self.db.insert("chain_height", &block.index.to_be_bytes()).unwrap();
        self.db.flush().unwrap(); // Ép ghi xuống ổ cứng ngay
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = format!("block:{}", index);
        if let Ok(Some(value)) = self.db.get(key) {
            return serde_json::from_slice(&value).ok();
        }
        None
    }

    pub fn get_height(&self) -> u64 {
        if let Ok(Some(val)) = self.db.get("chain_height") {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&val);
            return u64::from_be_bytes(arr);
        }
        0 // Genesis
    }

    // --- AI Knowledge Persistence (Thay thế learn_fact demo cũ) ---
    pub fn learn_fact(&self, key: &str, value: &str) {
        let db_key = format!("fact:{}", key);
        self.db.insert(db_key, value.as_bytes()).unwrap();
    }

    pub fn recall_fact(&self, key: &str) -> Option<String> {
        let db_key = format!("fact:{}", key);
        if let Ok(Some(val)) = self.db.get(db_key) {
            return Some(from_utf8(&val).unwrap().to_string());
        }
        None
    }
}
