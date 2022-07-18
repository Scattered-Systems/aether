/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use arguments::*;
pub use subs::*;

mod arguments;
mod subs;

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Acme")]
pub struct Commander {
    #[clap(arg_enum)]
    pub args: Args,
}
