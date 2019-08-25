#![crate_name = "blockchain_example"]

pub mod api;
pub mod blockchain;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use uuid::Uuid;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let block = blockchain.new_block(100, Some("2"));
    println!("{}", blockchain::Blockchain::hash(&block));
    println!("{}", blockchain::Blockchain::proof_of_work(1));
    // TODO: make chain shared across threads
    let sharedchain = web::Data::new(Mutex::new(blockchain));
    let node_identifier = web::Data::new(Uuid::new_v4().to_simple().to_string());

    HttpServer::new(move || {
        App::new()
            .register_data(sharedchain.clone())
            .register_data(node_identifier.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/mine").route(web::get().to(api::mine)))
            .service(web::resource("/transactions/new").route(web::post().to(api::new_transaction)))
            .service(web::resource("/chain").route(web::get().to(api::chain)))
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run();
}
