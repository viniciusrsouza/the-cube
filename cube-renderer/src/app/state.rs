pub struct Window {
    pub width: u32,
    pub height: u32,
}

pub struct State {
    pub window: Window,
}

pub enum AppState {
    State(State),
    None,
}
