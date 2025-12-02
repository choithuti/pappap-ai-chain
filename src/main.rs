// src/main.rs
mod constants;
mod ethics;
mod core;
mod ai;
mod network;
mod persona {
    pub mod membrane { pub mod signal_sanitizer; }
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::Mutex;
use actix_web::{App, HttpServer, web, middleware};
use libp2p::identity;

// Import các struct từ module
use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO};
use crate::ai::{cache::SmartCache, snn_core::SNNCore, trainer::AutoTrainer};
use crate::network::{p2p::P2PNode, webnode::WebNodeManager};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Khởi tạo Logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    println!("\n==================================================");
    println!("   🌌 PAPPAP AI - HOLY MEMBRANE NODE v7.7.7");
    println!("==================================================\n");

    // 2. Tầng Dữ liệu (Persistance Layer)
    // File storage.json sẽ được tạo tại thư mục gốc
    let storage = Arc::new(Storage::new("pappap_memory.json"));
    let cache = SmartCache::new();
    let dao = Arc::new(NeuroDAO::new());
    let wn_mgr = Arc::new(WebNodeManager::new());
    let peer_count = Arc::new(AtomicUsize::new(0));

    // 3. Tầng Mạng (P2P Layer - Libp2p)
    let local_key = identity::Keypair::generate_ed25519();
    let (mut p2p_node, mut p2p_rx, local_peer_id) = P2PNode::new(local_key, peer_count.clone())
        .await
        .expect("Failed to start P2P");
    
    println!("🆔 NODE PEER ID: {}", local_peer_id);
    let p2p_arc = Arc::new(Mutex::new(p2p_node));

    // 4. Tầng AI & Blockchain (The Core & Soul)
    // SNNCore dùng Storage thật để nhớ và Cache để truy xuất nhanh
    let snn_core = Arc::new(SNNCore::new(storage.clone(), cache.clone()));
    
    // Chain kết nối với Storage và AI
    // Lưu ý: Cần update PappapChain trong chain.rs để nhận SNNCore (nếu chưa khớp)
    // Ở đây ta giả định Chain dùng chung Storage với AI
    let chain = Arc::new(PappapChain::new(storage.clone(), cache.clone()).await);

    // 5. Kiểm tra hệ thống (System Check)
    if let Err(e) = crate::ethics::EthicsFilter::check("System Start") {
        panic!("❌ Ethics Filter Init Failed: {}", e);
    }
    println!("🧬 ETHICS FILTER: ACTIVE");

    // 6. Kích hoạt các luồng bất đồng bộ (Background Tasks)
    
    // Task A: P2P Network Runner
    let p2p_run = p2p_arc.clone();
    tokio::spawn(async move { 
        p2p_run.lock().await.run().await; 
    });

    // Task B: Chain Consensus (Mining/Validating)
    let chain_run = chain.clone();
    tokio::spawn(async move { 
        chain_run.run().await; 
    });

    // Task C: AI Dreaming (Auto-Training)
    let snn_trainer = snn_core.clone();
    tokio::spawn(async move {
        AutoTrainer::start(snn_trainer).await;
    });

    // Task D: WebNode Pruning (Dọn dẹp worker offline)
    let wn_pruner = wn_mgr.clone();
    tokio::spawn(async move {
        wn_pruner.prune_offline().await;
    });

    println!("🔮 ALL SYSTEMS GREEN. HOLY MEMBRANE SECURE.");

    // 7. API Gateway (REST API cho Frontend/Mobile)
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Inject các State vào API để Controller sử dụng
            .app_data(web::Data::new(chain.clone()))
            .app_data(web::Data::new(dao.clone()))
            .app_data(web::Data::new(wn_mgr.clone()))
            .app_data(web::Data::new(snn_core.clone()))
            // Route mặc định kiểm tra sức khỏe
            .route("/", web::get().to(|| async { "Pappap AI Node is Running" }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
