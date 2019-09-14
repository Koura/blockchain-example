use crate::api::Chain;
use chrono::{DateTime, Utc};
use crypto_hash::{hex_digest, Algorithm};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use urlparse::urlparse;

#[derive(Clone, Hash, Serialize, Deserialize, Debug)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Clone, Hash, Serialize, Deserialize, Debug)]
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
    pub nodes: HashSet<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transaction: vec![],
            nodes: HashSet::new(),
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

    /// Add a new node to the list of nodes
    ///
    /// :param address: Address of the node. Eg. 'http://192.168.0.5:5000'
    ///
    pub fn register_node(&mut self, address: &str) {
        let parsed_url = urlparse(address);
        self.nodes.insert(parsed_url.netloc);
    }

    /// Determine if a given blockchain is valid
    fn valid_chain(&self, chain: &[Block]) -> bool {
        let mut last_block = &chain[0];
        let mut current_index: usize = 1;
        while current_index < chain.len() {
            let block = &chain[current_index];
            println!("{:?}", last_block);
            println!("{:?}", block);
            println!("-----------");
            if block.previous_hash != Blockchain::hash(last_block) {
                return false;
            }
            if !Blockchain::valid_proof(last_block.proof, block.proof) {
                return false;
            }

            last_block = block;
            current_index += 1;
        }
        true
    }

    /// This is our Consensus Algorithm, it resolves conflicts
    /// by replacing our chain with the longest one in the network.
    ///
    /// :return True if our chain was replaced and false otherwise
    pub fn resolve_conflicts(&mut self) -> bool {
        let mut max_length = self.chain.len();
        let mut new_chain: Option<Vec<Block>> = None;

        // Grab and verify the chains from all the nodes in our network
        for node in &self.nodes {
            let mut response = reqwest::get(&format!("http://{}/chain", node)).unwrap();
            if response.status().is_success() {
                let node_chain: Chain = response.json().unwrap();
                if node_chain.length > max_length && self.valid_chain(&node_chain.chain) {
                    max_length = node_chain.length;
                    new_chain = Some(node_chain.chain);
                }
            }
        }
        // Replace our chain if we discovered a new, valid chain longer than ours
        match new_chain {
            Some(x) => {
                self.chain = x;
                true
            }
            None => false,
        }
    }
}
