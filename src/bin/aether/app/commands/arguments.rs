/*
    Appellation: arguments <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(clap::ArgEnum, Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Args {
    Mainnet,
    Subnet,
    Testnet,
    Oracle,
}
