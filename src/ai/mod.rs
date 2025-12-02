pub mod snn;        // (Cũ)
pub mod snn_core;   // (Chính - BioNeuron)
pub mod cache;
pub mod tools;      // [MỚI]
pub mod trainer {   // Placeholder trainer
    use std::sync::Arc;
    use super::snn_core::SNNCore;
    pub struct AutoTrainer;
    impl AutoTrainer {
        pub async fn start(snn: Arc<SNNCore>) {
            println!("💤 AI SLEEP LEARNING MODE ACTIVE");
            loop {
                // Tự động train nhẹ khi rảnh
                snn.train_step(0.1).await;
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        }
    }
}
