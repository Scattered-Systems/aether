use libp2p::floodsub::Topic;
use serde::{Deserialize, Serialize};

pub mod chain;
pub mod network;

pub mod primitives {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
}