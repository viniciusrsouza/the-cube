use super::renderable::{Drawable, Renderable};

pub struct Entity {
    pub id: u32,
    pub renderable: Option<Renderable>,
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
}

impl Drawable for Entity {
    fn draw(&self, gl: &web_sys::WebGl2RenderingContext) {
        let renderable = self.renderable.as_ref().unwrap();
        renderable.draw(gl);
    }
}
