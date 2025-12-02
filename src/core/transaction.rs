// src/core/transaction.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,   // Hex Public Key
    pub receiver: String, // Hex Address
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub signature: String, // Hex Signature
}

impl Transaction {
    pub fn calculate_hash(&self) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.sender, self.receiver, self.amount, self.fee, self.nonce, self.timestamp
        );
        let mut hasher = Sha256::new();
        hasher.update(payload);
        hex::encode(hasher.finalize())
    }

    pub fn verify(&self) -> bool {
        // 1. Decode Sender (Public Key)
        let pub_bytes = match hex::decode(&self.sender) {
            Ok(b) => b,
            Err(_) => return false,
        };
        
        let pub_key = match VerifyingKey::from_bytes(pub_bytes.as_slice().try_into().unwrap()) {
            Ok(k) => k,
            Err(_) => return false,
        };

        // 2. Decode Signature
        let sig_bytes = match hex::decode(&self.signature) {
            Ok(b) => b,
            Err(_) => return false,
        };

        if sig_bytes.len() != 64 { return false; }
        
        let sig_arr: [u8; 64] = sig_bytes.try_into().unwrap();
        let signature = Signature::from_bytes(&sig_arr);

        // 3. Verify Payload
        let payload = format!(
            "{}:{}:{}:{}:{}:{}",
            self.sender, self.receiver, self.amount, self.fee, self.nonce, self.timestamp
        );

        pub_key.verify(payload.as_bytes(), &signature).is_ok()
    }
}

// --- MEMPOOL ---
// Nơi chứa các giao dịch chờ được đóng gói vào Block
#[derive(Clone)]
pub struct Mempool {
    pub pending: Arc<RwLock<HashMap<String, Transaction>>>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { 
            pending: Arc::new(RwLock::new(HashMap::new())) 
        }
    }

    pub fn add_tx(&self, tx: Transaction) -> bool {
        // Verify ngay tại cửa ngõ
        if !tx.verify() {
            println!("⚠️ Invalid Transaction Signature: {}", tx.id);
            return false;
        }

        let mut pool = self.pending.write().unwrap();
        if pool.contains_key(&tx.id) {
            return false;
        }
        
        println!("📥 Mempool received TX: {}", tx.id);
        pool.insert(tx.id.clone(), tx);
        true
    }

    /// Lấy n giao dịch để Miner đóng gói
    pub fn pop_n(&self, n: usize) -> Vec<Transaction> {
        let mut pool = self.pending.write().unwrap();
        let keys: Vec<String> = pool.keys().take(n).cloned().collect();
        let mut txs = Vec::new();
        
        for key in keys {
            if let Some(tx) = pool.remove(&key) {
                txs.push(tx);
            }
        }
        txs
    }
    
    pub fn size(&self) -> usize {
        self.pending.read().unwrap().len()
    }
}
