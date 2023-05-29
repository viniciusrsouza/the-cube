use std::sync::MutexGuard;

use web_sys::WebGl2RenderingContext;

use crate::app::AppState;

use super::{
    behaviour::Behaviour,
    renderable::Renderable,
    transition::{self},
    DrawableContext, Light,
};

pub struct Entity {
    pub id: u32,
    renderable: Option<Renderable>,
    behaviour: Option<Box<dyn Behaviour>>,
    state: EntityState,
}

impl Entity {
    pub fn new(position: glm::Vec3) -> Self {
        Self {
            id: 0,
            renderable: None,
            behaviour: None,
            state: EntityState::new(position, glm::vec3(0.0, 0.0, 0.0)),
        }
    }

    pub fn register(&mut self, id: u32) {
        self.id = id;
    }

    pub fn add_renderable(&mut self, renderable: Renderable) {
        self.renderable = Some(renderable);
    }

    pub fn is_renderable(&self) -> bool {
        self.renderable.is_some()
    }

    pub fn add_behaviour(&mut self, behaviour: Box<dyn Behaviour>) {
        self.behaviour = Some(behaviour);
    }

    pub fn is_light_source(&self) -> bool {
        self.renderable
            .as_ref()
            .and_then(|r| Some(r.is_light_source()))
            .or(Some(false))
            .unwrap()
    }

    pub fn get_light(&mut self) -> Option<Light> {
        self.renderable.as_mut().and_then(|r| {
            r.light
                .as_mut()
                .and_then(|light| Some(light.with_position(self.state.position)).copied())
        })
    }

    pub fn update(&mut self, dt: f32, state: &mut MutexGuard<AppState>) {
        if let Some(behaviour) = self.behaviour.as_mut() {
            behaviour.update(dt, &mut self.state, state);
        }
    }

    pub fn draw<'a>(
        &'a mut self,
        gl: &WebGl2RenderingContext,
        ctx: &mut DrawableContext<'a>,
        dt: f32,
    ) {
        let renderable = self.renderable.as_mut().unwrap();
        self.state.sync_with_renderable(renderable);
        renderable.draw(gl, ctx, dt);
    }
}

pub struct EntityState {
    position: glm::Vec3,
    rotation: glm::Vec3,
    is_dirty: bool,
}

impl EntityState {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3) -> Self {
        Self {
            position,
            rotation,
            is_dirty: true,
        }
    }

    pub fn sync_with_renderable(&mut self, renderable: &mut Renderable) {
        if !self.is_dirty {
            return;
        }

        renderable.translate(self.position);
        // renderable.rotate(self.rotation);
        renderable.smooth_rotate(self.rotation, 100.0, transition::easing::ease_in_out);

        self.is_dirty = false;
    }

    pub fn set_position(&mut self, position: glm::Vec3) {
        self.position = position;
        self.is_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
        self.is_dirty = true;
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn get_rotation(&self) -> glm::Vec3 {
        self.rotation
    }
}
