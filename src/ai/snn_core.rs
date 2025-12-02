// src/ai/snn_core.rs
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::core::storage::Storage;
use crate::ai::cache::SmartCache;
use crate::ai::tools::{Oracle, LLMBridge};
use crate::ethics::EthicsFilter;
use crate::ai::snn::{SNN, DNum}; // [FIX] Import Deterministic SNN
use fixed::types::I48F16;

pub struct SNNCore {
    // Thay thế Vec<BioNeuron> bằng SNN struct chuẩn
    network: Arc<SNN>, 
    storage: Arc<Storage>,
    oracle: Oracle,
    llm: LLMBridge,
    cache: SmartCache,
}

impl SNNCore {
    pub fn new(storage: Arc<Storage>, cache: SmartCache) -> Self {
        println!("🧠 SNN CORE: INITIALIZED (Deterministic Mode)");
        Self {
            network: Arc::new(SNN::new()), // Khởi tạo mạng nơ-ron chuẩn
            storage,
            oracle: Oracle::new(),
            llm: LLMBridge::new(),
            cache,
        }
    }

    /// Tính toán điểm Spike Score (Consensus Critical)
    pub async fn forward(&self, intensity: f32) -> f32 {
        // [FIX] Chuyển đổi f32 sang DNum (Fixed Point)
        let input_val = DNum::from_num(intensity);
        
        // Tạo vector input (giả sử 64 input node lấy cùng giá trị)
        let inputs = vec![input_val; 64];
        
        // Lấy block height hiện tại để làm tham số thời gian (Determinism)
        let height = self.storage.get_height();

        // Chạy mạng nơ-ron
        let outputs = self.network.process(inputs, height);

        // Tính tổng output spike (đếm số lượng nơ-ron output kích hoạt > 0)
        let total_spike: DNum = outputs.iter().sum();
        
        // Chuyển về f32 để lưu vào Block (chỉ để hiển thị, logic core vẫn là fixed)
        total_spike.to_num::<f32>()
    }

    pub async fn process_text(&self, text: &str) -> (f32, String, String) {
        // (Giữ nguyên logic xử lý text, cache, oracle...)
        if let Err(e) = EthicsFilter::check(text) {
            return (0.0, "⛔ REJECTED".into(), e);
        }
        
        // ... (Logic cũ)
        if let Some(ans) = self.cache.get(text).await {
            return (100.0, "⚡ CACHE".into(), ans);
        }
        
        // Demo: Gọi forward để lấy điểm số thực tế
        let score = self.forward(0.8).await; 
        
        // ... (Placeholder response)
        (score, "🤖 AI".into(), "Processed".to_string())
    }
}
