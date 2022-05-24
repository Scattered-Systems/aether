use axum;

pub struct State;

enum States {
    Activating,
    Computing,
    Configuring,
    Connecting,
    Initializing,
    Inoperable,
    Operational,
    Synchronizing,
}

#[derive(Clone, Debug)]
pub struct Interface {
    settings: crate::settings::Settings,
}

impl Interface {
    pub fn new(settings: crate::settings::Settings) -> Self {
        Self {
            settings
        }
    }
}