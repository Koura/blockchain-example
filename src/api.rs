use crate::blockchain::{Block, Blockchain};

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Serialize)]
pub struct Chain {
    chain: Vec<Block>,
    length: usize,
}

pub fn new_transaction(
    state: web::Data<Mutex<Blockchain>>,
    req: web::Json<TransactionRequest>,
) -> HttpResponse {
    let sender = req.sender.to_owned();
    let recipient = req.recipient.to_owned();
    let index = state
        .lock()
        .unwrap()
        .new_transaction(&sender, &recipient, req.amount);
    HttpResponse::Created().json(TransactionResponse {
        message: format! {"Transaction will be added to Block {}", index},
    })
}

pub fn mine(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Mining a new block")
}

pub fn chain(state: web::Data<Mutex<Blockchain>>, _req: HttpRequest) -> HttpResponse {
    let length = state.lock().unwrap().chain.len();
    HttpResponse::Ok().json(Chain {
        chain: state.lock().unwrap().chain.clone(),
        length: length,
    })
}
