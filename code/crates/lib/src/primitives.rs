use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: acme::data::Bid,
    pub key: String,
    pub hashes: [String; 2],
    pub nonce: String,
    pub timestamp: acme::data::Clock
}

impl Block {
    pub fn new(key: String, previous_hash: String) -> Self {
        Self {
            id: acme::data::Bid::new(),
            key,
            hashes: [previous_hash, String::from("")],
            nonce: "".to_string(),
            timestamp: chrono::Local::now().into()
        }
    }
}