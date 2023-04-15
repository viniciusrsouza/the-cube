#[derive(Debug)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct AppState {
    pub viewport: Option<Viewport>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { viewport: None }
    }
}
