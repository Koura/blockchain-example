use crate::blockchain::{Block, Blockchain};

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Info {
    message: String,
}

#[derive(Serialize)]
pub struct Chain<'a> {
    chain: Vec<Block<'a>>,
    length: usize,
}

pub fn new_transaction(req: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().json(req.0)
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
