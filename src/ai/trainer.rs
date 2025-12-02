// src/ai/trainer.rs
use std::sync::Arc;
use crate::ai::snn_core::SNNCore;
use tokio::time::{sleep, Duration};

pub struct AutoTrainer;

impl AutoTrainer {
    pub async fn start(snn: Arc<SNNCore>) {
        println!("🏋️ AUTO TRAINER: STARTED (STDP Protocol Active)");
        
        loop {
            // 1. Kích thích ngẫu nhiên (Dreaming)
            // Trong lúc hệ thống rảnh rỗi, AI tự suy nghĩ về các vấn đề ngẫu nhiên
            // để củng cố các kết nối neuron.
            
            // Input thấp để mô phỏng trạng thái ngủ (REM sleep)
            let _activity = snn.forward(0.1).await;

            // 2. Bảo trì (Homeostasis)
            // Nếu neuron hoạt động quá mức, trainer sẽ giảm độ nhạy (trong thực tế)
            // Ở đây ta chỉ in log demo
            // println!("💤 AI Dreaming... Activity Level: {:.4}", _activity);

            // 3. Chu kỳ ngủ 10 giây
            sleep(Duration::from_secs(10)).await;
        }
    }
}
