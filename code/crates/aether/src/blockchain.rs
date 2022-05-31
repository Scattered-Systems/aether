use acme::primitives::{Clock, Identity};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub previous: String,
    pub nonce: String,
    pub timestamp: Clock,
}

impl Block {
    pub fn new(hash: String, previous: String, nonce: String) -> Self {
        Self {
            id: ObjectId::new(),
            hash,
            previous,
            nonce,
            timestamp: Local::now().into(),
        }
    }
}

