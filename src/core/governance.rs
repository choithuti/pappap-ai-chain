use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub votes_yes: u64,
    pub votes_no: u64,
    pub status: String, // "Active", "Passed", "Rejected"
}

pub struct NeuroDAO {
    proposals: RwLock<HashMap<u64, Proposal>>,
    next_id: RwLock<u64>,
}

impl NeuroDAO {
    pub fn new() -> Self {
        println!("⚖️  NEURO DAO GOVERNANCE SYSTEM ONLINE");
        Self {
            proposals: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        }
    }

    pub async fn create_proposal(&self, title: String, desc: String) -> u64 {
        let mut id_lock = self.next_id.write().await;
        let id = *id_lock;
        *id_lock += 1;

        let prop = Proposal {
            id, title, description: desc, votes_yes: 0, votes_no: 0, status: "Active".to_string(),
        };

        self.proposals.write().await.insert(id, prop);
        id
    }
    // ... (Giữ nguyên phần còn lại của file governance.rs bạn đã upload)
}
