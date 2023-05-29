use self::config::Config;
pub use self::{
    keyboard::{from_key_code, modifiers, Key, Keyboard},
    viewport::Viewport,
};

mod config;
mod keyboard;
mod viewport;

#[derive(Debug)]
pub struct AppState {
    pub viewport: Option<Viewport>,
    pub keyboard: Keyboard,
    pub config: Config,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            viewport: None,
            keyboard: Keyboard::new(),
            config: Config::new("".to_string()),
        }
    }
}
