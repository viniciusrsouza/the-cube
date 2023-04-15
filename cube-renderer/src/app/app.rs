use std::{ops::Deref, sync::MutexGuard};

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    model::{Drawable, EntityBuffer},
    utils::{window, Instant},
};

use super::AppState;

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

        let app = App {
            canvas,
            gl,
            entities: EntityBuffer::new(),
            now: Instant::now(),
        };

        Ok(app)
    }

    pub fn render(&mut self, state: MutexGuard<AppState>) {
        let dt = self.now.elapsed() as f32;

        self.update(dt, state);
        self.draw(dt);
    }

    fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update(&mut self, dt: f32, state: MutexGuard<AppState>) {}
    pub fn draw(&mut self, dt: f32) {
        self.clear();
        self.entities.draw(&self.gl);
    }
}
