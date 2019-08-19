#![crate_name = "blockchain_example"]

pub mod api;
pub mod blockchain;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let block = blockchain.new_block(100, Some("2"));
    println!("{}", blockchain::Blockchain::hash(&block));
    println!("{}", blockchain::Blockchain::proof_of_work(1));
    // TODO: make chain shared across threads
    let sharedchain = web::Data::new(Mutex::new(blockchain));

    HttpServer::new(move || {
        App::new()
            .register_data(sharedchain.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/mine").route(web::get().to(api::mine)))
            .service(web::resource("/transactions/new").route(web::post().to(api::new_transaction)))
            .service(web::resource("/chain").route(web::get().to(api::chain)))
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run();
}
