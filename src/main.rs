#![crate_name = "blockchain_example"]

pub mod blockchain;

use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let block = blockchain.new_block(100, Some("2"));
    println!("{}", blockchain::Blockchain::hash(&block));
    println!("{}", blockchain::Blockchain::proof_of_work(1));

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
