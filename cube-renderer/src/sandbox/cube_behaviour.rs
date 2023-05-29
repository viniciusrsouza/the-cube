use std::sync::MutexGuard;

use crate::{
    app::AppState,
    model::{Behaviour, EntityState},
    network::WebSocket,
    utils::{Message, Transform},
    HANDLE,
};

pub struct CubeBehaviour {
    conn: WebSocket,
}

impl CubeBehaviour {
    pub fn new() -> Self {
        let host;
        {
            let state = HANDLE.lock().unwrap();
            host = state.config.host.clone();
        }

        let url = format!("ws://{}:8080/ws", host);
        Self {
            conn: WebSocket::new(url, "cube"),
        }
    }
}

impl Behaviour for CubeBehaviour {
    fn update(&mut self, _dt: f32, entity: &mut EntityState, _state: &mut MutexGuard<AppState>) {
        for message in self.conn.poll() {
            match message {
                Message::Transform(transform) => match transform {
                    Transform::Rotate(rotation) => entity.set_rotation(rotation),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
