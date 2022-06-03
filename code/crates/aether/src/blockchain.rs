use acme::primitives::{Clock, ObjectId, Transaction};
use chrono::Local;

#[derive(Clone, Debug)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub previous: String,
    pub nonce: String,
    pub timestamp: Clock,
    pub transactions: [Transaction; 16],
}

impl Block {
    pub fn new(hash: String, previous: String, nonce: String, transactions: [Transaction; 16]) -> Self {
        let id = ObjectId::new();
        let timestamp: Clock = Local::now().into();
        Self {
            id,
            hash,
            previous,
            nonce,
            timestamp,
            transactions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chain {
    pub id: ObjectId,
    pub key: String,
    pub blocks: Vec<Block>,
    pub version: String,
}