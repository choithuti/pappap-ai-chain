// src/main.rs
mod constants;
mod ethics;
mod evolution { pub mod ghost_cell_orchestrator; }
mod core {
    pub mod block; pub mod chain; pub mod transaction;
    pub mod wallet; pub mod storage; pub mod governance;
}
mod ai {
    pub mod snn; pub mod snn_core; pub mod cache;
    pub mod tools; pub mod trainer;
}
mod network { pub mod p2p; pub mod webnode; }
mod persona {
    pub mod membrane { pub mod signal_sanitizer; }
    pub mod symbiosis { pub mod render_params; }
}
mod api { pub mod routes; pub mod mod; }

// [FIX] Khai báo module Genetics đúng cách
pub mod genetics {
    include!(concat!(env!("OUT_DIR"), "/pappap.genetics.rs"));
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::{Mutex, mpsc};
use actix_web::{App, HttpServer, web, middleware};
use libp2p::identity;

use crate::evolution::ghost_cell_orchestrator::GhostCellOrchestrator;
use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO, transaction::Mempool};
use crate::ai::{cache::SmartCache, snn_core::SNNCore, trainer::AutoTrainer};
use crate::network::{p2p::P2PNode, webnode::WebNodeManager};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    println!("🌌 PAPPAP AI NODE v0.8.1 (AUDITED)");

    // 1. GENETICS CHECK
    let ghost_cell = GhostCellOrchestrator::new(1700000000);
    if !ghost_cell.check_vitality() { panic!("💀 GHOST CELL EXPIRED"); }

    // 2. DATA
    let storage = Arc::new(Storage::new("pappap_v1.db"));
    let mempool = Arc::new(Mempool::new());
    let cache = SmartCache::new();
    let dao = Arc::new(NeuroDAO::new());
    let wn_mgr = Arc::new(WebNodeManager::new());

    // 3. NETWORK (P2P)
    let local_key = identity::Keypair::generate_ed25519();
    let peer_count = Arc::new(AtomicUsize::new(0));

    // [FIX] Nhận về p2p_sender (command channel) thay vì receiver
    let (mut p2p_node, p2p_sender, local_peer_id) = P2PNode::new(local_key, peer_count.clone())
        .await
        .expect("P2P Init Failed");
    
    let p2p_arc = Arc::new(Mutex::new(p2p_node));
    println!("🆔 NODE ID: {}", local_peer_id);

    // 4. AI & CHAIN
    let snn_core = Arc::new(SNNCore::new(storage.clone(), cache.clone()));
    
    let chain = Arc::new(PappapChain::new(
        storage.clone(),
        mempool.clone(),
        snn_core.clone(),
        p2p_sender, // Truyền Sender vào Chain
    ).await);

    // 5. TASKS

    // Task A: P2P Runner (Logic đã được đưa vào trong struct P2PNode)
    let p2p_runner = p2p_arc.clone();
    tokio::spawn(async move {
        p2p_runner.lock().await.run().await;
    });

    // Task B: Mining
    let chain_miner = chain.clone();
    tokio::spawn(async move { chain_miner.run().await; });

    // Task C: Training
    let ai_trainer = snn_core.clone();
    tokio::spawn(async move { AutoTrainer::start(ai_trainer).await; });

    // Task D: WebNode Pruning
    let wn_pruner = wn_mgr.clone();
    tokio::spawn(async move { wn_pruner.prune_offline().await; });

    // 6. API
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(actix_cors::Cors::permissive()) // Cho phép Frontend gọi API
            // Inject State
            .app_data(web::Data::new(chain.clone()))
            .app_data(web::Data::new(mempool.clone()))
            .app_data(web::Data::new(dao.clone()))
            .app_data(web::Data::new(wn_mgr.clone()))
            .app_data(web::Data::new(snn_core.clone()))
            // .app_data(web::Data::new(peer_count.clone())) // Nếu cần hiển thị peers
            // Load Routes từ module API
            .configure(crate::api::routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
