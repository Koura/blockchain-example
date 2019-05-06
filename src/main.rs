#![crate_name = "blockchain_example"]

pub mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let block = blockchain.new_block(100, Some("2"));
    println!("{}", blockchain::Blockchain::hash(&block));
}
