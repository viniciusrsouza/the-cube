mod app;
mod camera;
mod model;
mod resources;
mod sandbox;
mod utils;

use std::{cell::RefCell, rc::Rc, sync::Mutex};

use app::{App, AppState};
use sandbox::{load_shaders, make_cube, make_lights};
use utils::window;
use wasm_bindgen::prelude::*;

use crate::{app::Viewport, utils::request_animation_frame};

extern crate console_error_panic_hook;
extern crate nalgebra_glm as glm;
use std::panic;

lazy_static::lazy_static! {
    static ref HANDLE: Mutex<AppState> = Mutex::new(AppState::new());
}

#[wasm_bindgen(start)]
fn on_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

fn start_loop(mut app: App) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    console::info!("Starting render loop");
    *g.borrow_mut() = Some(Closure::new(move || {
        app.render(HANDLE.lock().unwrap());
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    let mut app = App::new()?;
    init_events()?;

    load_shaders(&mut app)
        .await
        .expect("Failed to load shaders");
    make_cube(&mut app);
    make_lights(&mut app);

    start_loop(app);
    Ok(())
}

fn init_events() -> Result<(), JsValue> {
    let resize_callback = Closure::<dyn FnMut()>::new(move || {
        let window = window();
        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        let mut state = HANDLE.lock().unwrap();
        state.viewport = Some(Viewport { width, height });
    });
    window()
        .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())?;
    resize_callback.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    console::log!("Cube initialized");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn trace(a: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn clear();
}

#[allow(unused_macros, unused_imports)]
pub mod console {
    macro_rules! log {
        ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
    }

    macro_rules! info {
        ($($t:tt)*) => (crate::info(&format_args!($($t)*).to_string()))
    }

    macro_rules! error {
        ($($t:tt)*) => (crate::error(&format_args!($($t)*).to_string()))
    }

    macro_rules! _warn {
        ($($t:tt)*) => (crate::warn(&format_args!($($t)*).to_string()))
    }

    macro_rules! debug {
        ($($t:tt)*) => (crate::debug(&format_args!($($t)*).to_string()))
    }

    macro_rules! trace {
        ($($t:tt)*) => (crate::trace(&format_args!($($t)*).to_string()))
    }

    macro_rules! clear {
        () => {
            crate::clear()
        };
    }

    pub(crate) use _warn as warn;
    pub(crate) use clear;
    pub(crate) use debug;
    pub(crate) use error;
    pub(crate) use info;
    pub(crate) use log;
    pub(crate) use trace;
}
