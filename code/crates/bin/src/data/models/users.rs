use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name {
    pub complete: String,
    pub title: String,
    pub prefix: String,
    pub first: String,
    pub middle: String,
    pub last: String,
    pub suffix: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    id: bson::oid::ObjectId,
    username: String,
    created_at: bson::DateTime,
    modified_at: bson::DateTime,
}

impl User {
    pub fn new(username: String) -> Self {
        let id = bson::oid::ObjectId::new();
        let created_at = chrono::Local::now().into();
        Self {
            id,
            username,
            created_at,
            modified_at: created_at,
        }
    }
}