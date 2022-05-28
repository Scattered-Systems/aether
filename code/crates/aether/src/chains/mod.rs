mod blocks;

type Object = std::collections::HashMap<String, Box<String>>;

pub struct Keys {
    pub timestamp: bson::DateTime,
}

pub trait Appchain {
    fn get(&self) -> Chain;
}

enum OperatingStates {
    Initializing,
    Initialized,
    Terminated,
}

pub struct Chain {
    pub id: u8,
    pub key: String,
    pub blocks: Vec<crate::chains::blocks::Block>,
}

impl Chain {
    fn new(id: u8, key: String, blocks: Vec<crate::chains::blocks::Block>) -> Self {
        Self {
            id,
            key,
            blocks,
        }
    }
}