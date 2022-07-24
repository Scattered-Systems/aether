/*
   Appellation: index
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use axum::{routing::get, Json, Router};
use std::collections::HashMap;

pub type Dictionary<T> = HashMap<String, T>;
pub type Container<T> = Dictionary<Vec<T>>;

pub fn create_route() -> Router {
    axum::Router::new().route("/", get(current_time))
}

pub async fn current_time() -> Json<serde_json::Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    let timestamp: bson::DateTime = chrono::Local::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    Json(serde_json::json!(cache))
}
