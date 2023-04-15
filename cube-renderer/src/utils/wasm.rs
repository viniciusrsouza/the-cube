use wasm_bindgen::prelude::*;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn now() -> f64 {
    let perf = window().performance();
    match perf {
        Some(perf) => perf.now(),
        None => 0.,
    }
}

pub struct Instant {
    start: f64,
}

impl Instant {
    pub fn now() -> Self {
        Self { start: now() }
    }

    pub fn elapsed(&self) -> f64 {
        now() - self.start
    }
}
