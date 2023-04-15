use web_sys::WebGl2RenderingContext;

pub struct Renderable {
    pub id: u32,
}

impl Drawable for Renderable {
    fn draw(&self, gl: &WebGl2RenderingContext) {}
}

pub trait Drawable: Sized {
    fn draw(&self, gl: &WebGl2RenderingContext);
}
