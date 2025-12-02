// src/evolution/ghost_cell_orchestrator.rs
use std::time::{SystemTime, UNIX_EPOCH};
use crate::constants::GHOST_CELL_DEATH;

pub struct GhostCellOrchestrator {
    born_at: u64,
}

impl GhostCellOrchestrator {
    /// Khởi tạo tế bào ma với thời điểm sinh ra (Genesis Timestamp)
    pub fn new(genesis_timestamp: u64) -> Self {
        Self { born_at: genesis_timestamp }
    }

    /// Kiểm tra sinh hiệu. Trả về true nếu còn sống, false nếu đã chết già.
    pub fn check_vitality(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let age = now.saturating_sub(self.born_at);

        if age > GHOST_CELL_DEATH {
            println!("⚰️  GHOST CELL DEATH: Age {}s > Limit {}s. Initiating Apoptosis.", age, GHOST_CELL_DEATH);
            return false;
        }
        
        true
    }

    /// Trả về thời gian còn lại (giây)
    pub fn time_remaining(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let death_time = self.born_at + GHOST_CELL_DEATH;
        
        if now >= death_time {
            0
        } else {
            death_time - now
        }
    }
}
