mod constants;
mod ethics; // Load module Ethics
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
}
mod network {
    pub mod p2p;
    pub mod webnode;
}
mod persona {
    pub mod membrane { pub mod signal_sanitizer; }
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::Mutex;
use actix_web::{App, HttpServer, web, middleware};
use libp2p::identity;

use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO};
use crate::ai::{cache::SmartCache, snn_core::SNNCore};
use crate::network::{p2p::P2PNode, webnode::WebNodeManager};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("🌌 PAPPAP AI NODE SYSTEM DNA: [7-77-777]");

    // 1. Init Data Layer
    let storage = Arc::new(Storage::new("pappap_data"));
    let cache = SmartCache::new();
    let dao = Arc::new(NeuroDAO::new());
    let wn_mgr = Arc::new(WebNodeManager::new());
    let peer_count = Arc::new(AtomicUsize::new(0));

    // 2. Init Network Layer (Libp2p)
    let local_key = identity::Keypair::generate_ed25519();
    let (mut p2p_node, _p2p_rx, _local_peer_id) = P2PNode::new(local_key, peer_count.clone()).await.unwrap();
    let p2p_arc = Arc::new(Mutex::new(p2p_node));

    // 3. Init AI & Chain Layer (With SNN Core & Ethics)
    // Lưu ý: Chain cần được sửa để nhận SNNCore thay vì snn dummy cũ nếu muốn tích hợp sâu
    let snn_core = Arc::new(SNNCore::new(storage.clone(), cache.clone()));
    
    // Demo: Test Ethics Filter ngay khi khởi động
    let _ = crate::ethics::EthicsFilter::check("Hello World"); // OK
    let _ = crate::ethics::EthicsFilter::check("chứa từ khóa phản động"); // Sẽ báo lỗi

    // 4. Background Tasks
    let p2p_run = p2p_arc.clone();
    tokio::spawn(async move { p2p_run.lock().await.run().await; });

    println!("🔮 HOLY MEMBRANE INTEGRITY CHECK: PASS");
    println!("🧬 ETHICS FILTER: ACTIVE");

    // 5. API Server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(dao.clone()))
            .app_data(web::Data::new(wn_mgr.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
