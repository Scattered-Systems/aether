/*
   Appellation: core
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use crate::core::{common::*, context::*, settings::*};

mod context;
mod settings;

mod common {
    pub use constants::*;
    pub use types::*;
    pub use utils::*;

    mod constants {}

    mod types {}

    mod utils {}
}
