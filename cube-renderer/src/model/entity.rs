use crate::resources::Assets;

use super::renderable::{Drawable, Renderable};

pub struct Entity {
    pub id: u32,
    renderable: Option<Renderable>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            id: 0,
            renderable: None,
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
}

impl Drawable for Entity {
    fn draw(&self, gl: &web_sys::WebGl2RenderingContext, assets: &Assets) {
        let renderable = self.renderable.as_ref().unwrap();
        renderable.draw(gl, assets);
    }
}
