mod blocks;

type Object = std::collections::HashMap<String, Box<String>>;

pub struct Keys {
    pub timestamp: bson::DateTime
}



pub enum Chains {
    Application,
    Mainnet,
    Subnet
}

pub struct Chain {
    pub id: u8,
    pub keys: String,
    pub blocks: Vec<Block>
}

impl Chains::Application for Chain {
    fn new() {

    }
}