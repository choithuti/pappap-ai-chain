// src/ai/snn_core.rs
use tokio::sync::RwLock;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use crate::core::storage::Storage;
use crate::ai::cache::SmartCache;
use crate::ai::tools::{Oracle, LLMBridge};
use crate::ethics::EthicsFilter;

#[derive(Clone, Debug)]
pub struct BioNeuron {
    pub potential: f32,
    pub threshold: f32,
    pub decay: f32,
    pub sensitivity: f32,
}

pub struct SNNCore {
    neurons: RwLock<Vec<BioNeuron>>,
    storage: Arc<Storage>,
    oracle: Oracle,
    llm: LLMBridge,
    cache: SmartCache,
}

impl SNNCore {
    pub fn new(storage: Arc<Storage>, cache: SmartCache) -> Self {
        let mut rng = rand::thread_rng();
        // Giảm số lượng neuron xuống 1000 cho bản demo nhẹ nhàng
        let neuron_count = 1000;
        println!("🧠 SNN CORE ONLINE | Initialized {} Bio-Neurons", neuron_count);

        let mut neurons = Vec::with_capacity(neuron_count);
        for _ in 0..neuron_count {
            neurons.push(BioNeuron {
                potential: -70.0, // Điện thế nghỉ
                threshold: -55.0, // Ngưỡng kích hoạt
                decay: 0.95,      // Hệ số suy hao
                sensitivity: rng.gen_range(0.8..1.2),
            });
        }

        Self {
            neurons: RwLock::new(neurons),
            storage,
            oracle: Oracle::new(),
            llm: LLMBridge::new(),
            cache,
        }
    }

    /// Mô phỏng quá trình Forward (Lan truyền) để tính điểm thông minh
    pub async fn forward(&self, intensity: f32) -> f32 {
        let mut neurons = self.neurons.write().await;
        let mut rng = rand::thread_rng();
        let mut active_count = 0.0;

        for n in neurons.iter_mut() {
            // Tích hợp tín hiệu đầu vào + nhiễu ngẫu nhiên (Noise)
            let noise = rng.gen_range(-0.5..0.5);
            n.potential += (intensity * n.sensitivity) + noise;

            // Kiểm tra ngưỡng Spike
            if n.potential >= n.threshold {
                n.potential = -85.0; // Hyperpolarization (Quá phân cực sau khi bắn)
                active_count += 1.0;
            } else {
                // Suy hao tự nhiên theo thời gian
                n.potential *= n.decay;
            }
        }
        
        // Trả về "Spike Score" = Tổng số neuron kích hoạt / 10
        active_count / 10.0
    }

    /// Xử lý text input (Hỏi đáp AI)
    pub async fn process_text(&self, text: &str) -> (f32, String, String) {
        // 1. Kiểm duyệt nội dung
        if let Err(e) = EthicsFilter::check(text) {
            return (0.0, "⛔ REJECTED".into(), e);
        }

        // 2. Check Cache
        if let Some(ans) = self.cache.get(text).await {
            return (100.0, "⚡ CACHE".into(), ans);
        }

        // 3. Check Memory (DB)
        if let Some(ans) = self.storage.recall_fact(text) {
            return (80.0, "🧠 MEMORY".into(), ans);
        }

        // 4. Ask Oracle / LLM
        let mut ans = String::new();
        if let Ok(res) = self.oracle.smart_search(text).await {
            if !res.contains("Offline") && !res.contains("No results") {
                ans = res;
            }
        }
        
        if ans.is_empty() {
            if let Ok(res) = self.llm.ask_ai(text).await {
                ans = res;
            }
        }

        if ans.is_empty() {
            ans = "I tried to think, but the void answered back.".to_string();
        } else {
            // Học kiến thức mới
            self.storage.learn_fact(text, &ans);
            self.cache.set(text.into(), ans.clone()).await;
        }

        (50.0, "🤖 THINKING".into(), ans)
    }
}
