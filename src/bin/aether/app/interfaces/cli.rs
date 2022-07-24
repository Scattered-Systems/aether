/*
    Appellation: cli <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::commands::{Args, SubCommands};

#[derive(clap::Parser, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Aether is a zk-SNARK Ethereum Native Multi-Chain")]
pub struct AetherCLI {
    #[clap(arg_enum)]
    pub args: Args,
    #[clap(subcommand)]
    pub context: SubCommands,
}
