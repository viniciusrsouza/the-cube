use std::sync::MutexGuard;

use crate::app::AppState;

use super::entity::EntityState;

pub trait Behaviour {
    fn update(&mut self, dt: f32, entity: &mut EntityState, state: &mut MutexGuard<AppState>);
}
