use axum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    name: String,
    slug: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Provider {
    endpoint: String,
    public_key: String,
    secret_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    host: [u8; 4],
    port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub project: Project,
    pub providers: Box<Provider>,
    pub server: Server,
}


pub struct Interface {
    settings: Settings,
}