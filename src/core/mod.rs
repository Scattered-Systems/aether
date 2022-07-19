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

    mod types {
        pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
        /// Describes a boxed dynamic error
        pub type StandardError = Box<dyn std::error::Error>;
        /// Describes a configuration builder in their default state
        pub type DefaultConfigBuilder = config::ConfigBuilder<config::builder::DefaultState>;
        /// Describes the result of a collection of configuration files
        pub type CnfFiles = Vec<config::File<config::FileSourceFile, config::FileFormat>>;
    }

    mod utils {
        pub fn collect_config_files(pattern: &str, required: bool) -> crate::CnfFiles {
            let f = |pat: &str, opt: bool| {
                glob::glob(pat)
                    .unwrap()
                    .map(|path| config::File::from(path.unwrap()).required(opt))
                    .collect::<Vec<_>>()
            };
            f(pattern, required)
        }
    }
}
