// src/api/routes.rs
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::core::chain::PappapChain;
use crate::core::transaction::{Mempool, Transaction};
use crate::core::governance::NeuroDAO;
use crate::ai::snn_core::SNNCore;
use crate::network::webnode::WebNodeManager;

// --- DTOs (Data Transfer Objects) ---

#[derive(Serialize)]
struct StatusResponse {
    node_version: String,
    height: u64,
    last_hash: String,
    peers: usize,
    mempool_size: usize,
}

#[derive(Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Serialize)]
struct ChatResponse {
    score: f32,
    source: String,
    answer: String,
}

#[derive(Deserialize)]
struct ProposalRequest {
    title: String,
    description: String,
}

// --- HANDLERS ---

/// GET /status - Kiểm tra trạng thái Node
async fn get_node_status(
    chain: web::Data<Arc<PappapChain>>,
    mempool: web::Data<Arc<Mempool>>,
    // Lưu ý: peer_count cần được inject từ main.rs nếu muốn hiển thị
) -> impl Responder {
    let height = chain.storage.get_height();
    let last_hash = chain.storage.get_last_hash();
    
    HttpResponse::Ok().json(StatusResponse {
        node_version: "0.8.1".to_string(),
        height,
        last_hash,
        peers: 0, // Cần inject peer_count atomic để lấy số thực
        mempool_size: mempool.size(),
    })
}

/// POST /tx - Gửi giao dịch mới
async fn submit_transaction(
    mempool: web::Data<Arc<Mempool>>,
    tx: web::Json<Transaction>,
) -> impl Responder {
    // 1. Validate Transaction (Signature, Format)
    if !tx.verify() {
        return HttpResponse::BadRequest().body("Invalid Signature");
    }

    // 2. Add to Mempool
    if mempool.add_tx(tx.into_inner()) {
        HttpResponse::Ok().body("Transaction Accepted")
    } else {
        HttpResponse::Conflict().body("Transaction already exists or invalid")
    }
}

/// POST /ai/chat - Trò chuyện với Pappap AI
async fn ask_ai(
    snn: web::Data<Arc<SNNCore>>,
    req: web::Json<ChatRequest>,
) -> impl Responder {
    let (score, src, ans) = snn.process_text(&req.prompt).await;
    
    HttpResponse::Ok().json(ChatResponse {
        score,
        source: src,
        answer: ans,
    })
}

/// GET /governance/proposals - Lấy danh sách đề xuất
async fn list_proposals(
    dao: web::Data<Arc<NeuroDAO>>,
) -> impl Responder {
    let props = dao.list_proposals().await;
    HttpResponse::Ok().json(props)
}

/// POST /governance/proposals - Tạo đề xuất mới
async fn create_proposal(
    dao: web::Data<Arc<NeuroDAO>>,
    req: web::Json<ProposalRequest>,
) -> impl Responder {
    let id = dao.create_proposal(req.title.clone(), req.description.clone()).await;
    HttpResponse::Ok().json(serde_json::json!({ "id": id, "status": "Created" }))
}

/// GET /webnodes - Lấy thống kê Web Workers
async fn get_webnodes(
    wn: web::Data<Arc<WebNodeManager>>,
) -> impl Responder {
    let (count, power) = wn.get_stats().await;
    HttpResponse::Ok().json(serde_json::json!({ 
        "active_workers": count, 
        "total_hashrate": power 
    }))
}

// --- CONFIGURATOR ---

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/status", web::get().to(get_node_status))
            .route("/tx", web::post().to(submit_transaction))
            .route("/ai/chat", web::post().to(ask_ai))
            .route("/governance/proposals", web::get().to(list_proposals))
            .route("/governance/proposals", web::post().to(create_proposal))
            .route("/webnodes", web::get().to(get_webnodes))
    );
}
