// src/core/chain.rs
use crate::constants::FEEDBACK_TIMEOUT_MS;
use crate::core::block::Block;
use crate::core::storage::Storage;
use crate::core::transaction::Mempool;
use crate::ai::snn_core::SNNCore;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::UnboundedSender;

pub struct PappapChain {
    pub storage: Arc<Storage>,
    pub mempool: Arc<Mempool>,
    pub snn: Arc<SNNCore>,
    pub p2p_sender: UnboundedSender<Vec<u8>>, // Kênh để bắn Block ra mạng P2P
}

impl PappapChain {
    pub async fn new(
        storage: Arc<Storage>, 
        mempool: Arc<Mempool>,
        snn: Arc<SNNCore>,
        p2p_sender: UnboundedSender<Vec<u8>>
    ) -> Self {
        Self { storage, mempool, snn, p2p_sender }
    }

    pub async fn run(&self) {
        println!("⛏️  MINING ENGINE STARTED: Waiting for transactions...");
        
        loop {
            // 1. Kiểm tra Mempool xem có đủ giao dịch để đóng block không
            // Ở đây demo lấy ít nhất 1 giao dịch, thực tế có thể đào block rỗng
            if self.mempool.size() == 0 {
                sleep(Duration::from_millis(1000)).await;
                continue;
            }

            let txs = self.mempool.pop_n(10); // Lấy tối đa 10 tx
            println!("⚡ Mining Block with {} transactions...", txs.len());

            // 2. Lấy thông tin Chain hiện tại
            let height = self.storage.get_height() + 1;
            let last_hash = self.storage.get_last_hash();

            // 3. AI Consensus (Proof of Intelligence)
            // AI phải tính toán một giá trị "Spike" dựa trên trạng thái mạng
            // Đây là bước thay thế Proof of Work (đốt điện)
            let spike_val = self.snn.forward(0.5).await; // 0.5 là input kích thích

            // 4. Tạo Block mới
            let new_block = Block::new(
                height,
                last_hash,
                txs,
                "Local_Miner_01".to_string(), // Tên miner
                spike_val
            );

            // 5. Lưu Block vào Storage
            self.storage.save_block(&new_block);
            
            println!("✅ BLOCK #{} MINED | Hash: {} | Spike: {}", 
                height, 
                &new_block.hash[0..16], // In ngắn gọn
                spike_val
            );

            // 6. Broadcast Block ra mạng P2P
            if let Ok(block_bytes) = serde_json::to_vec(&new_block) {
                if let Err(e) = self.p2p_sender.send(block_bytes) {
                    println!("⚠️ Failed to broadcast block: {}", e);
                }
            }

            // 7. Nghỉ ngơi theo nhịp sinh học (Feedback Timeout)
            sleep(Duration::from_millis(FEEDBACK_TIMEOUT_MS)).await;
        }
    }
}
