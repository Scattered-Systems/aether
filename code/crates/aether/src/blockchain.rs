use acme::data::{Clock, Identity};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Block {
    pub id: Identity,
    pub created: Clock,
    pub hash: String,
    // Block Hash
    pub previous: String,
    // Previous Hash
    pub nonce: String,
}

impl Block {
    pub fn new(hash: String, previous: String, nonce: String) -> Self {
        Self {
            id: Identity::new(),
            created: Local::now().into(),
            hash,
            previous,
            nonce,
        }
    }
}

pub trait Chain {
    fn genesis(&self) -> Box<Block>;
}

enum OperatingStates {
    Initializing,
    Initialized,
    Terminated,
}

pub struct Interface {
    pub blocks: Box<Block>,
}

impl Interface {
    fn new(blocks: Box<Block>) -> Self {
        Self {
            blocks,
        }
    }
}

impl Chain for Interface {
    fn genesis(&self) -> Box<Block> {
        todo!()
    }
}