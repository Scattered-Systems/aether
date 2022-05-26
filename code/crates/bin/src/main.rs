use crate::core::interface::Interface;

mod api;
mod chains;
mod clients;
mod core;
mod data;

#[tokio::main]
async fn main() {
    Interface::new().await;
}