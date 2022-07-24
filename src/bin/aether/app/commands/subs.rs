/*
    Appellation: subs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(clap::Subcommand, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SubCommands {
    Archive {
        #[clap(default_value = "", long, short, value_parser)]
        address: String,
    },
}
