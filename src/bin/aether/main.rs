/*
   Appellation: aether <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::{app::*, core::*, data::*};

mod app;
mod core;
mod data;

#[tokio::main]
async fn main() -> Result<(), scsys::BoxError> {
    Aether::api().await;
    Ok(())
}
