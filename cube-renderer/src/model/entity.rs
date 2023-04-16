use web_sys::WebGl2RenderingContext;

use super::{
    renderable::{Renderable, Transform},
    DrawableContext, Light,
};

pub struct Entity {
    pub id: u32,
    position: glm::Vec3,
    renderable: Option<Renderable>,
    rotation: glm::Vec3,
}

impl Entity {
    pub fn new(position: glm::Vec3) -> Self {
        Self {
            id: 0,
            position,
            renderable: None,
            rotation: glm::vec3(0.0, 0.0, 0.0),
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

    pub fn update(&mut self, dt: f32) {
        self.rotation = glm::vec3(0.0, 0.0025 * dt, 0.0);
    }

    pub fn draw<'a>(&'a mut self, gl: &WebGl2RenderingContext, ctx: &mut DrawableContext<'a>) {
        let renderable = self.renderable.as_mut().unwrap();
        renderable
            .with_transform(Transform::Translate(self.position))
            .with_transform(Transform::Rotate(self.rotation))
            .draw(gl, ctx);
    }
}
