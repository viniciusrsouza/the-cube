use web_sys::WebGl2RenderingContext;

use crate::console;

use super::{
    renderable::Renderable,
    transition::{self},
    DrawableContext, Light,
};

pub struct Entity {
    pub id: u32,
    position: glm::Vec3,
    rotation: glm::Vec3,
    renderable: Option<Renderable>,
    is_dirty: bool,
}

impl Entity {
    pub fn new(position: glm::Vec3) -> Self {
        Self {
            id: 0,
            position,
            renderable: None,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            is_dirty: true,
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
                .and_then(|light| Some(light.with_position(self.position)).copied())
        })
    }

    pub fn update(&mut self, dt: f32) {}

    pub fn draw<'a>(
        &'a mut self,
        gl: &WebGl2RenderingContext,
        ctx: &mut DrawableContext<'a>,
        dt: f32,
    ) {
        self.sync_with_renderer();
        let renderable = self.renderable.as_mut().unwrap();
        renderable.draw(gl, ctx, dt);
    }

    fn sync_with_renderer(&mut self) {
        if !self.is_dirty {
            return;
        }

        let renderable = self.renderable.as_mut().unwrap();
        renderable.translate(self.position);
        renderable.smooth_rotate(self.rotation, 300.0, transition::easing::ease_in_out);

        self.is_dirty = false;
    }
}
