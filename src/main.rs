// src/main.rs
mod api;
mod core;
mod ai;
mod network;
mod config;
mod persona {
    pub mod membrane { pub mod signal_sanitizer; }
}

use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::Mutex;
use actix_web::{App, HttpServer, web, middleware};
use env_logger;

use crate::core::{chain::PappapChain, storage::Storage, governance::NeuroDAO};
use crate::ai::{snn::SNNCore, trainer::AutoTrainer, cache::SmartCache};
use crate::network::webnode::WebNodeManager;
use crate::persona::membrane::signal_sanitizer::RenderParams;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("🌌 PAPPAP AI NODE INITIALIZING...");

    // 1. Kiểm tra cấu trúc vật lý
    let storage = Arc::new(Storage::new("pappap_data"));
    let cache = SmartCache::new();
    
    // 2. Khởi tạo Chain và Membrane Check
    // Lưu ý: P2PNode tạm thời để None hoặc khởi tạo thực tế tùy context
    let chain = Arc::new(PappapChain::new(storage.clone(), cache, /* p2p_arc */).await);

    // 3. Khởi động các luồng bất tử
    let chain_run = chain.clone();
    tokio::spawn(async move { 
        chain_run.run().await; 
    });

    println!("🔮 HOLY MEMBRANE v7.7.7 LOADED");
    println!("📜 Genesis: {} bytes | Air Gap: {} bytes", crate::constants::GENESIS_SIZE, crate::constants::AIR_GAP_SIZE);
    println!("⚡ Eternal Signature: {:?}", crate::constants::ETERNAL_SIGNATURE);
    if std::mem::size_of::<RenderParams>() != 64 {
    panic!("CRITICAL: RenderParams size violation! Expected 64, got {}. Check struct alignment.", std::mem::size_of::<RenderParams>());
}
println!("✅ RenderParams Integrity: 64 bytes locked.");

    // 4. API Server (Nếu cần)
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(chain.clone()))
    })
    .bind("127.0.0.1:8080")? // Bind tạm
    .run()
    .await
}
