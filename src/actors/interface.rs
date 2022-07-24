/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Status {
    pub key: Vec<String>,
    pub message: Vec<String>,
    pub subject: String,
    pub title: String,
    pub timestamp: i64,
}

impl Status {
    fn constructor(
        key: Vec<String>,
        message: Vec<String>,
        subject: String,
        title: String,
        timestamp: i64,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            key,
            message,
            subject,
            title,
            timestamp,
        })
    }

    pub fn new(key: Vec<String>, message: Vec<String>, subject: String, title: String) -> Self {
        match Self::constructor(key, message, subject, title, chrono::Utc::now().timestamp()) {
            Ok(v) => v,
            Err(e) => panic!("Status Error: {}", e),
        }
    }

    pub fn init() -> Self {
        Self::new(Vec::new(), Vec::new(), "".to_string(), "".to_string())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum NodeStatus {
    Inoperable(Status),
    Operational(Status),
    Operating(Status),
    Terminating(Status),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Networks {
    Mainnet,
    Subnet,
    Testnet,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AetherNode {
    pub network: Networks,
    pub status: NodeStatus,
}
