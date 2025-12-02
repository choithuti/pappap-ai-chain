mod constants;
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
    pub mod snn_core; // File bạn đã upload trước đó
    pub mod trainer { pub struct AutoTrainer; impl AutoTrainer { pub async fn start(_: std::sync::Arc<super::snn::SNNCore>) {} } }
    pub mod cache;
}
mod network {
    pub mod p2p;
    pub mod webnode; // File bạn đã upload trước đó
}
mod persona {
    pub mod membrane { pub mod signal_sanitizer; }
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::Mutex;
use actix_web::{App, HttpServer, web, middleware};
use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO};
use crate::ai::{cache::SmartCache, trainer::AutoTrainer};
use crate::network::{p2p::P2PNode, webnode::WebNodeManager};
use libp2p::identity;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("🌌 PAPPAP AI NODE INITIATING SEQUENCE...");

    // Init Core Components
    let storage = Arc::new(Storage::new("pappap_data"));
    let cache = SmartCache::new();
    let wn_mgr = Arc::new(WebNodeManager::new());
    let peer_count = Arc::new(AtomicUsize::new(0));

    // Network
    let local_key = identity::Keypair::generate_ed25519();
    let (mut p2p_node, p2p_rx, local_peer_id) = P2PNode::new(local_key, peer_count.clone(), 7777).await.unwrap();
    let p2p_arc = Arc::new(Mutex::new(p2p_node));

    // AI Chain
    let chain = Arc::new(PappapChain::new(storage.clone(), cache).await);

    // Spawn Background Tasks
    let chain_run = chain.clone();
    let p2p_run = p2p_arc.clone();
    
    tokio::spawn(async move { p2p_run.lock().await.run(p2p_rx).await; });
    tokio::spawn(async move { chain_run.run().await; });
    tokio::spawn(async move { AutoTrainer::start(chain.snn.clone()).await; });

    println!("🔮 HOLY MEMBRANE v7.7.7 ACTIVE");
    
    // API Server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(wn_mgr.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
