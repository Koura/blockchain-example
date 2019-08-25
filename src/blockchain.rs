use chrono::{DateTime, Utc};
use crypto_hash::{hex_digest, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

#[derive(Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transaction: vec![],
        };
        blockchain.new_block(100, Some("1"));
        blockchain
    }
    /// Create a new Block in the Blockchain
    ///
    /// :param proof: The proof given by the Proof of Work algorithm
    /// :param previous_hash: (Optional) hash of previous Block
    /// :return: New Block
    pub fn new_block(&mut self, proof: u64, previous_hash: Option<&str>) -> Block {
        let block = Block {
            index: (self.chain.len() + 1) as u64,
            timestamp: Utc::now(),
            transactions: self.current_transaction.drain(0..).collect(),
            proof,
            previous_hash: previous_hash.unwrap_or("0").to_string(),
        };

        self.chain.push(block.clone());
        block
    }
    /// Creates a new transaction to go into the next mined Block
    ///
    /// :param sender: Address of the śender
    /// :param recipient: Address of the recipient
    /// :param amount: Amount
    /// :return: The index of the Block that will hold this transaction
    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: i64) -> u64 {
        self.current_transaction.push(Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
        });
        self.last_block().unwrap().index + 1
    }
    /// Simple Proof of Work Algorithm:
    /// - Find a number p' such that hash(pp') contains 4 leading zeroes,
    ///   where p is the previous proof, and p' is the new proof
    pub fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof = 0;
        while !Self::valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }
    /// Validates the Proof: Does hash(last_proof, proof) containt 4 leading zeroes?
    fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = hex_digest(Algorithm::SHA256, guess.as_bytes());
        guess_hash.ends_with("0000")
    }

    /// Creates a SHA-256 hash of a Block
    ///
    /// :param block: Block
    /// :return hash for the block
    pub fn hash(block: &Block) -> String {
        let serialized = serde_json::to_string(&block).unwrap();
        hex_digest(Algorithm::SHA256, serialized.as_bytes())
    }
    /// Returns the last Block in the chain
    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}
