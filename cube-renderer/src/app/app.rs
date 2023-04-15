use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    model::{Drawable, EntityBuffer},
    utils::Instant,
};

pub struct App {
    pub canvas: HtmlCanvasElement,
    pub gl: WebGl2RenderingContext,

    pub entities: EntityBuffer,
    now: Instant,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

        let gl = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        Ok(App {
            canvas,
            gl,
            entities: EntityBuffer::new(),
            now: Instant::now(),
        })
    }

    pub fn render(&mut self) {
        let dt = self.now.elapsed() as f32;

        self.update(dt);
        self.draw(dt);
    }

    fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update(&mut self, dt: f32) {}
    pub fn draw(&mut self, dt: f32) {
        self.clear();
        self.entities.draw(&self.gl);
    }
}
