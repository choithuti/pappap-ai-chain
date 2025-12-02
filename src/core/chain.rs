// src/core/chain.rs
use std::sync::Arc;
use std::time::{Instant, Duration};
use crate::constants::*;
use crate::ai::snn::SNNCore; //
use crate::core::storage::Storage;
use tokio::sync::Mutex;

pub struct PappapChain {
    pub storage: Arc<Storage>,
    pub snn: Arc<SNNCore>,
    // ... các field khác
}

impl PappapChain {
    pub async fn new(storage: Arc<Storage>, _cache: crate::ai::cache::SmartCache, _p2p: Arc<Mutex<crate::network::p2p::P2PNode>>) -> Self {
        Self {
            storage,
            snn: Arc::new(SNNCore::new()), //
        }
    }

    pub async fn validate_holy_membrane(&self) -> bool {
        // Load từ đường dẫn cấu trúc mới
        let genesis_path = "core/bootstrap/genesis_reader.wasm";
        let air_gap_path = "persona/membrane/air_gap.wasm";

        let genesis = match std::fs::read(genesis_path) {
            Ok(data) => data,
            Err(_) => { println!("❌ MISSING GENESIS: {}", genesis_path); return false; }
        };

        let air_gap = match std::fs::read(air_gap_path) {
            Ok(data) => data,
            Err(_) => { println!("❌ MISSING AIR_GAP: {}", air_gap_path); return false; }
        };

        // Kiểm tra kích thước byte chính xác đến từng đơn vị
        if genesis.len() as u64 != GENESIS_SIZE {
            println!("⚠️ GENESIS SIZE VIOLATION: {} ≠ {}", genesis.len(), GENESIS_SIZE);
            return false;
        }
        if air_gap.len() as u64 != AIR_GAP_SIZE {
            println!("⚠️ AIR_GAP VIOLATION: {} ≠ {}", air_gap.len(), AIR_GAP_SIZE);
            return false;
        }
        
        println!("✅ HOLY MEMBRANE INTEGRITY: 100%");
        true
    }

    pub async fn run(&self) {
        assert!(self.validate_holy_membrane().await, "🛑 HOLY MEMBRANE COMPROMISED – SHUTTING DOWN UNIVERSE");

        loop {
            let height = self.storage.get_height() + 1;
            
            // AI tính toán spike (Deterministic)
            let _spike = self.snn.deterministic_forward(0.0, height).await;

            // Kiểm tra Gene cấm và Ghost Cell
            if height > 7 && FORBIDDEN_GENES.contains(&height) {
                 println!("⚠️ FORBIDDEN GENE DETECTED AT BLOCK {}", height);
            }

            if height > 7 && height % 777_777 == 0 {
                if Instant::now().elapsed().as_secs() > GHOST_CELL_DEATH {
                    panic!("☠️ Ghost Cell awakened after 7 years – Terminating.");
                }
            }

            // Timeout feedback loop
            tokio::time::sleep(Duration::from_millis(FEEDBACK_TIMEOUT_MS)).await;
        }
    }
}
