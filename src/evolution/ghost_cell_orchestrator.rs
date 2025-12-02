// src/evolution/ghost_cell_orchestrator.rs
use std::time::{SystemTime, UNIX_EPOCH};
use crate::constants::GHOST_CELL_DEATH; //

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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let age = now - self.born_at;

        if age > GHOST_CELL_DEATH {
            println!("⚰️  GHOST CELL DEATH: Age {}s > Limit {}s. Initiating Shutdown.", age, GHOST_CELL_DEATH);
            return false;
        }
        
        // Logic phụ: Kiểm tra xem có bị đánh thức quá sớm không (Gene 777777)
        // Nếu height < 777777 mà force awaken -> False
        
        true
    }

    pub fn time_remaining(&self) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if now > self.born_at + GHOST_CELL_DEATH {
            0
        } else {
            (self.born_at + GHOST_CELL_DEATH) - now
        }
    }
}
