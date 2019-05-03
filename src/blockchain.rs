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
