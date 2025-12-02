// src/network/webnode.rs
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct WebWorker {
    pub client_id: String,
    pub last_seen: u64,
    pub hashrate: f32,
    pub reputation: u8,
}

pub struct WebNodeManager {
    workers: RwLock<HashMap<String, WebWorker>>,
}

impl WebNodeManager {
    pub fn new() -> Self {
        println!("🌐 WEB NODE MANAGER: ONLINE");
        Self {
            workers: RwLock::new(HashMap::new()),
        }
    }

    /// Worker gửi nhịp tim (Ping) để báo cáo trạng thái
    pub async fn register_beat(&self, client_id: String, hashrate: f32) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut w = self.workers.write().await;
        
        w.insert(client_id.clone(), WebWorker { 
            client_id, 
            last_seen: now, 
            hashrate,
            reputation: 100 // Default score
        });
    }

    /// Lấy thống kê mạng lưới WebNode
    pub async fn get_stats(&self) -> (usize, f32) {
        let w = self.workers.read().await;
        let count = w.len();
        let total_power: f32 = w.values().map(|v| v.hashrate).sum();
        (count, total_power)
    }

    /// Loại bỏ các node offline quá 15 giây
    pub async fn prune_offline(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let mut w = self.workers.write().await;
            let len_before = w.len();
            
            w.retain(|_, v| now - v.last_seen < 15);
            
            let removed = len_before - w.len();
            if removed > 0 {
                println!("🍂 Pruned {} offline WebNodes", removed);
            }
        }
    }
}
