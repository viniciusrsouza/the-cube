use std::sync::MutexGuard;

use crate::{
    app::AppState,
    model::{Behaviour, EntityState},
    network::WebSocket,
    utils::{Message, Transform},
};

pub struct CubeBehaviour {
    conn: WebSocket,
}

impl CubeBehaviour {
    pub fn new() -> Self {
        Self {
            conn: WebSocket::new("ws://localhost:8080/ws", "cube"),
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
