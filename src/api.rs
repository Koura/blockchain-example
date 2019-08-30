use crate::blockchain::{Block, Blockchain, Transaction};

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

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
pub struct MiningRespose {
    message: String,
    index: u64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

#[derive(Serialize)]
pub struct Chain {
    chain: Vec<Block>,
    length: usize,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    total_nodes: usize,
}

#[derive(Serialize)]
pub struct ResolveResponse {
    message: String,
    chain: Vec<Block>,
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

pub fn mine(
    node_identifier: web::Data<String>,
    state: web::Data<Mutex<Blockchain>>,
    _req: HttpRequest,
) -> HttpResponse {
    let (proof, previous_hash) = {
        let mut blockchain = state.lock().unwrap();
        let last_block = blockchain.last_block().unwrap();
        let last_proof = last_block.proof;
        let proof = Blockchain::proof_of_work(last_proof);
        let previous_hash = Blockchain::hash(last_block);
        (proof, previous_hash)
    };
    let mut blockchain = state.lock().unwrap();
    blockchain.new_transaction("0", &*node_identifier, 1);
    let block = blockchain.new_block(proof, Some(&previous_hash));
    HttpResponse::Ok().json(MiningRespose {
        message: "New Block Forged".to_string(),
        index: block.index,
        transactions: block.transactions,
        proof: proof,
        previous_hash: previous_hash,
    })
}

pub fn chain(state: web::Data<Mutex<Blockchain>>, _req: HttpRequest) -> HttpResponse {
    let length = state.lock().unwrap().chain.len();
    HttpResponse::Ok().json(Chain {
        chain: state.lock().unwrap().chain.clone(),
        length: length,
    })
}

pub fn register_node(reg: HttpRequest) -> HttpResponse {
    HttpResponse::Created().json(RegisterResponse {
        message: "hello world".to_string(),
        total_nodes: 3,
    })
}

pub fn resolve_nodes(__req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ResolveResponse {
        message: "hello world".to_string(),
        chain: Vec::new(),
    })
}
