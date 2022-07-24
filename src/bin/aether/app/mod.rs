/*
   Appellation: chaos <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use clap::Parser;

pub mod commands;
pub mod endpoints;
pub mod interfaces;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Aether;

impl Aether {
    pub async fn api() {
        let mut api = interfaces::AetherAPI::new();
        println!("{}", &api);
        api.run().await
    }
    pub fn cli() -> interfaces::AetherCLI {
        interfaces::AetherCLI::parse()
    }
}
