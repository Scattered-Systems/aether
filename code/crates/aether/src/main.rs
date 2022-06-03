use crate::app::application::Application;

mod app;
mod blockchain;
mod data;

#[tokio::main]
async fn main() {
    Application::new().await;
}
