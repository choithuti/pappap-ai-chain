// src/main.rs

// --- 1. IMPORT MODULES ---
mod constants;
mod ethics;
mod evolution {
    pub mod ghost_cell_orchestrator;
}
mod core {
    pub mod block;
    pub mod chain;
    pub mod transaction;
    pub mod wallet;
    pub mod storage;
    pub mod governance;
}
mod ai {
    pub mod snn;
    pub mod snn_core;
    pub mod cache;
    pub mod tools;
    pub mod trainer;
}
mod network {
    pub mod p2p;
    pub mod webnode;
}
mod persona {
    pub mod membrane {
        pub mod signal_sanitizer;
    }
    pub mod symbiosis {
        pub mod render_params;
    }
}

// Import code Protobuf được sinh ra từ build.rs
pub mod genetics {
    include!(concat!(env!("OUT_DIR"), "/pappap.genetics.rs"));
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::{Mutex, mpsc};
use actix_web::{App, HttpServer, web, middleware};
use libp2p::identity;

// Import structs
use crate::evolution::ghost_cell_orchestrator::GhostCellOrchestrator;
use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO, transaction::Mempool};
use crate::ai::{cache::SmartCache, snn_core::SNNCore, trainer::AutoTrainer};
use crate::network::{p2p::P2PNode, webnode::WebNodeManager};
use crate::persona::symbiosis::render_params::RenderParams;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 0. Setup Logger & Env
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    println!("\n==================================================");
    println!("   🌌 PAPPAP AI - HOLY MEMBRANE NODE v0.8.0");
    println!("==================================================\n");

    // 1. GENETICS & EVOLUTION CHECK
    // Kiểm tra System DNA và Ghost Cell trước khi khởi động bất cứ thứ gì
    let genesis_time = 1700000000; // Thay bằng timestamp thực tế của block genesis
    let ghost_cell = GhostCellOrchestrator::new(genesis_time);
    
    if !ghost_cell.check_vitality() {
        panic!("💀 CRITICAL: Ghost Cell has expired. The Era is over.");
    }
    println!("🧬 SYSTEM DNA: VALID | ⏳ Ghost Cell Time Remaining: {}s", ghost_cell.time_remaining());

    // 2. DATA LAYER (Storage & Mempool)
    let storage = Arc::new(Storage::new("pappap_v1.db"));
    let mempool = Arc::new(Mempool::new());
    let cache = SmartCache::new();
    let dao = Arc::new(NeuroDAO::new());
    let wn_mgr = Arc::new(WebNodeManager::new());

    // 3. NETWORK LAYER (P2P)
    // Channel để Chain gửi Block đã đào được ra P2P Network
    let (p2p_tx, mut p2p_rx_cmd) = mpsc::unbounded_channel::<Vec<u8>>();
    
    let local_key = identity::Keypair::generate_ed25519();
    let peer_count = Arc::new(AtomicUsize::new(0));
    
    // Khởi tạo P2P Node
    let (mut p2p_node, mut p2p_rx_net, local_peer_id) = P2PNode::new(local_key, peer_count.clone())
        .await
        .expect("Failed to initialize P2P Layer");
    
    let p2p_arc = Arc::new(Mutex::new(p2p_node));
    println!("🆔 NODE ID: {}", local_peer_id);

    // 4. AI & CHAIN LAYER
    let snn_core = Arc::new(SNNCore::new(storage.clone(), cache.clone()));
    
    // Chain kết nối với Storage, Mempool, AI và P2P Sender
    let chain = Arc::new(PappapChain::new(
        storage.clone(),
        mempool.clone(),
        snn_core.clone(),
        p2p_tx, // Trao quyền gửi block cho Chain
    ).await);

    // Verify RenderParams Size (Membrane Integrity Check)
    if std::mem::size_of::<RenderParams>() != 64 {
        panic!("❌ MEMBRANE RUPTURE: RenderParams size is {} bytes (Expected 64)", std::mem::size_of::<RenderParams>());
    }
    println!("🔮 HOLY MEMBRANE INTEGRITY: 100%");

    // 5. SPAWN BACKGROUND TASKS

    // Task A: P2P Network Runner
    let p2p_runner = p2p_arc.clone();
    tokio::spawn(async move {
        println!("🌐 P2P Loop Started");
        let mut node = p2p_runner.lock().await;
        loop {
            tokio::select! {
                // 1. Nhận broadcast lệnh từ Chain
                Some(block_data) = p2p_rx_cmd.recv() => {
                    node.broadcast_block(block_data);
                }
                // 2. Chạy vòng lặp P2P (xử lý kết nối, nhận tin...)
                // Lưu ý: Hàm run() trong p2p.rs cần được sửa thành run_step() hoặc async loop
                // Ở đây giả định p2p_node.run() đã được thiết kế lại để không chặn luồng
                 _ = node.run() => {} 
            }
        }
    });

    // Task B: Mining / Consensus
    let chain_miner = chain.clone();
    tokio::spawn(async move {
        chain_miner.run().await;
    });

    // Task C: AI Dreaming (Training)
    let ai_trainer = snn_core.clone();
    tokio::spawn(async move {
        AutoTrainer::start(ai_trainer).await;
    });

    // Task D: WebNode Pruning
    let wn_pruner = wn_mgr.clone();
    tokio::spawn(async move {
        wn_pruner.prune_offline().await;
    });

    // Task E: Ghost Cell Watchdog (Kiểm tra định kỳ mỗi giờ)
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            if !ghost_cell.check_vitality() {
                println!("💀 GHOST CELL EXPIRED DURING RUNTIME. SHUTTING DOWN.");
                std::process::exit(777);
            }
        }
    });

    // 6. API SERVER
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Inject State
            .app_data(web::Data::new(chain.clone()))
            .app_data(web::Data::new(mempool.clone()))
            .app_data(web::Data::new(dao.clone()))
            .app_data(web::Data::new(wn_mgr.clone()))
            .app_data(web::Data::new(snn_core.clone()))
            .app_data(web::Data::new(peer_count.clone()))
            // Default Route
            .route("/", web::get().to(|| async { "Pappap AI Node is Running [Active]" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
