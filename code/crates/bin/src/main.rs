mod app;
mod chains;
mod clients;
mod data;
mod settings;

#[tokio::main]
async fn main() {
    crate::app::application::Application::new().await;
}