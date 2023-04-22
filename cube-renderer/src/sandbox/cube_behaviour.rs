use std::sync::MutexGuard;

use crate::{
    app::{modifiers, AppState, Key},
    model::{Behaviour, EntityState},
};

pub struct CubeBehaviour {}

impl CubeBehaviour {
    pub fn new() -> Self {
        Self {}
    }
}

impl Behaviour for CubeBehaviour {
    fn update(&mut self, _dt: f32, entity: &mut EntityState, state: &mut MutexGuard<AppState>) {
        let rotation = entity.get_rotation();
        if state.keyboard.is_down(Key::Space, modifiers::SHIFT, true) {
            entity.set_rotation(glm::vec3(rotation.x + 0.5, rotation.y, rotation.z));
        } else if state.keyboard.is_down(Key::Space, modifiers::CTRL, true) {
            entity.set_rotation(glm::vec3(rotation.x, rotation.y + 0.5, rotation.z));
        } else if state.keyboard.is_down(Key::Space, 0, false) {
            entity.set_rotation(glm::vec3(rotation.x, rotation.y, rotation.z + 0.5));
        }
    }
}
