// src/core/storage.rs
use sled::{Db, IVec};
use std::str::from_utf8;
use serde::{Serialize, Deserialize};
use crate::core::block::Block;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        println!("💾 MOUNTING SLED DB AT: {}", path);
        // Sled tự động tạo thư mục và file db
        let db = sled::open(path).expect("Failed to open Sled database");
        Self { db }
    }

    // --- Block Methods ---

    pub fn save_block(&self, block: &Block) {
        // Key: "block:<index>"
        let key = format!("block:{}", block.index);
        let value = serde_json::to_vec(block).expect("Failed to serialize block");
        
        self.db.insert(key.as_bytes(), value).unwrap();
        
        // Cập nhật chiều cao và hash mới nhất
        self.db.insert("chain_height", &block.index.to_be_bytes()).unwrap();
        self.db.insert("last_hash", block.hash.as_bytes()).unwrap();
        
        // Flush để đảm bảo dữ liệu ghi xuống ổ cứng
        self.db.flush().unwrap();
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = format!("block:{}", index);
        if let Ok(Some(value)) = self.db.get(key.as_bytes()) {
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
        0 // Mặc định là 0 nếu chưa có block nào
    }

    pub fn get_last_hash(&self) -> String {
        if let Ok(Some(val)) = self.db.get("last_hash") {
            return String::from_utf8(val.to_vec()).unwrap_or_else(|_| "0".repeat(64));
        }
        "0".repeat(64) // Genesis prev_hash mặc định
    }

    // --- AI Knowledge Base (Key-Value) ---

    pub fn learn_fact(&self, key: &str, value: &str) {
        let db_key = format!("fact:{}", key);
        self.db.insert(db_key.as_bytes(), value.as_bytes()).unwrap();
    }

    pub fn recall_fact(&self, key: &str) -> Option<String> {
        let db_key = format!("fact:{}", key);
        if let Ok(Some(val)) = self.db.get(db_key.as_bytes()) {
            return Some(from_utf8(&val).unwrap_or("").to_string());
        }
        None
    }
}
