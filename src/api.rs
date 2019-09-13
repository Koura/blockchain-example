use crate::blockchain::{Block, Blockchain, Transaction};

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
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

#[derive(Serialize, Deserialize)]
pub struct Chain {
    pub chain: Vec<Block>,
    pub length: usize,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    nodes: Vec<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    total_nodes: Vec<String>,
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
    HttpResponse::Created().json(MessageResponse {
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

pub fn register_node(
    state: web::Data<Mutex<Blockchain>>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    if (req.nodes.is_empty()) {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: "Error: Please supply a valid list of nodes".to_string(),
        });
    }
    let mut blockchain = state.lock().unwrap();
    for node in req.nodes.iter() {
        blockchain.register_node(node)
    }
    HttpResponse::Created().json(RegisterResponse {
        message: "New nodes have been added".to_string(),
        total_nodes: blockchain.nodes.iter().cloned().collect(),
    })
}

pub fn resolve_nodes(state: web::Data<Mutex<Blockchain>>, _req: HttpRequest) -> HttpResponse {
    let mut blockchain = state.lock().unwrap();
    let replaced = blockchain.resolve_conflicts();
    let message = if replaced {
        "Our chain was replaced"
    } else {
        "Our chain is authorative"
    };
    HttpResponse::Ok().json(ResolveResponse {
        message: message.to_string(),
        chain: blockchain.chain.clone(),
    })
}
