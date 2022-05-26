use crate::api::interface::Interface;

mod api;
mod chains;
mod clients;
mod data;

#[tokio::main]
async fn main() {
    let settings = match crate::api::settings::Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    let app = Interface::new(settings).await;
    println!("{:#?}", app.context.settings)
}