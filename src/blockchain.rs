use chrono::{DateTime, Utc};

struct Transaction<'a> {
    sender: &'a str,
    recipient: &'a str,
    amount: i64,
}

struct Block<'a> {
    index: u64,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction<'a>>,
    proof: u64,
    previous_hash: &'a str,
}

#[derive(Default)]
pub struct Blockchain<T> {
    chain: Vec<T>,
    current_transaction: Vec<T>,
}

impl<T> Blockchain<T> {
    /// Creates a new Block and adds it to the chain
    pub fn new_block(&mut self) {}
    /// Adds a new transaction to the list of transactions
    pub fn new_transaction(&mut self) {}
    /// Hashes a Block
    pub fn hash(block: T) {}
    /// Returns the last Block in the chain
    pub fn last_block(self) {}
}
