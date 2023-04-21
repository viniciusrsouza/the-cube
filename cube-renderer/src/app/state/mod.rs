pub use self::{
    keyboard::{from_key_code, modifiers, Key, Keyboard},
    viewport::Viewport,
};

mod keyboard;
mod viewport;

#[derive(Debug)]
pub struct AppState {
    pub viewport: Option<Viewport>,
    pub keyboard: Keyboard,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            viewport: None,
            keyboard: Keyboard::new(),
        }
    }
}
