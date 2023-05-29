use std::sync::MutexGuard;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    camera::Camera,
    model::{DrawableContext, EntityBuffer},
    resources::Assets,
    utils::Instant,
    HANDLE,
};

use super::{AppState, Viewport};

pub struct App {
    pub canvas: HtmlCanvasElement,
    pub gl: WebGl2RenderingContext,

    pub entities: EntityBuffer,
    pub assets: Assets,
    now: Instant,
    camera: Camera,
}

impl App {
    pub fn new(host: String) -> Result<App, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

        {
            let mut state = HANDLE.lock().unwrap();
            state.config.host = host;
        }

        let gl = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        gl.enable(WebGl2RenderingContext::DEPTH_TEST);

        let app = App {
            canvas,
            gl,
            entities: EntityBuffer::new(),
            assets: Assets::new(),
            now: Instant::now(),
            camera: Camera::new(glm::vec3(0., 2.5, 5.), glm::vec3(0., 1., 0.), -90., -25.),
        };

        Ok(app)
    }

    pub fn render(&mut self, state: MutexGuard<AppState>) {
        let dt = self.now.elapsed() as f32;

        self.update(dt, state);
        self.draw(dt);

        self.now.reset();
    }

    fn clear(&self) {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn update(&mut self, dt: f32, mut state: MutexGuard<AppState>) {
        self.sync_state(&mut state);
        self.entities.update(dt, &mut state);
    }
    pub fn draw(&mut self, dt: f32) {
        let viewport = Viewport {
            width: self.canvas.width() as u32,
            height: self.canvas.height() as u32,
        };
        let mut ctx = DrawableContext::new(&self.gl, &self.camera, &self.assets, &viewport);
        self.clear();
        self.entities.draw(&self.gl, &mut ctx, dt);
    }

    fn sync_state(&mut self, state: &mut MutexGuard<AppState>) {
        if let Some(viewport) = state.viewport.take() {
            self.sync_viewport(&viewport);
            state.viewport = None;
        }
    }

    fn sync_viewport(&self, viewport: &Viewport) {
        self.gl
            .viewport(0, 0, viewport.width as i32, viewport.height as i32);
    }
}
