/*
   Appellation: acme <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{app::*, context::*, core::*, data::*};

mod app;
mod context;
mod core;
mod data;

pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), AsyncError> {
    let mut interface = Interface::new();
    println!("{}", &interface);
    interface.run().await;
    Ok(())
}
