// evolution/ghost_cell_orchestrator.rs
use std::time::{SystemTime, UNIX_EPOCH};

// 7 năm tính bằng giây
const GHOST_CELL_LIFESPAN: u64 = 220_752_000; 

pub struct GhostCell {
    pub born_at: u64,
}

impl GhostCell {
    pub fn new(born_at: u64) -> Self {
        Self { born_at }
    }

    pub fn check_vitality(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let age = now - self.born_at;

        if age > GHOST_CELL_LIFESPAN {
            // Kích hoạt Apoptosis (Tự chết của tế bào)
            eprintln!("⚰️  GHOST CELL EXPIRED. Age: {}s > Limit: {}s", age, GHOST_CELL_LIFESPAN);
            std::process::exit(777); // Mã lỗi đặc biệt cho cái chết tự nhiên
        } else {
            println!("👻 Ghost Cell active. Remaining: {}s", GHOST_CELL_LIFESPAN - age);
        }
    }
}
