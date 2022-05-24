use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Provider {
    pub endpoint: String,
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub databases: Box<Database>,
    pub project: Project,
    pub providers: Box<Provider>,
    pub server: Server,
}