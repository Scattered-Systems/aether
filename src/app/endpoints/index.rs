/*
   Appellation: index
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use axum;
use std::collections::HashMap;

pub type Dictionary<T> = HashMap<String, T>;
pub type Container<T> = Dictionary<Vec<T>>;

pub fn create_route() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(base))
}

pub async fn base() -> axum::Json<serde_json::Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    let timestamp: bson::DateTime = chrono::Local::now().into();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(serde_json::json!(cache))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
