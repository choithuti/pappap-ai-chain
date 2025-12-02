use std::collections::HashMap;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

struct CacheItem {
    value: String,
    expires_at: Instant,
}

#[derive(Clone)] // Cần derive Clone để dùng trong Arc/Share
pub struct SmartCache {
    store: Arc<RwLock<HashMap<String, CacheItem>>>, // Bọc trong Arc để Clone rẻ tiền
    ttl: Duration,
}

use std::sync::Arc;

impl SmartCache {
    pub fn new() -> Self {
        println!("⚡ SMART CACHE ACTIVATED (TTL: 1 Hour)");
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(3600),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let store = self.store.read().await;
        if let Some(item) = store.get(key) {
            if item.expires_at > Instant::now() {
                return Some(item.value.clone());
            }
        }
        None
    }

    pub async fn set(&self, key: String, value: String) {
        let mut store = self.store.write().await;
        store.insert(key, CacheItem {
            value,
            expires_at: Instant::now() + self.ttl,
        });
    }
}
