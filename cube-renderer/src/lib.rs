mod app;
mod camera;
mod model;
mod network;
mod resources;
mod sandbox;
mod utils;

use std::{cell::RefCell, rc::Rc, sync::Mutex};

use app::{from_key_code, App, AppState};
use sandbox::{load_shaders, make_cube, make_lights};
use utils::window;
use wasm_bindgen::prelude::*;

use crate::{
    app::{modifiers, Viewport},
    utils::request_animation_frame,
};

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
pub async fn run(host: String) -> Result<(), JsValue> {
    let mut app = App::new(host)?;
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
    let resize = Closure::wrap(Box::new(on_resize) as Box<dyn Fn()>);
    window().set_onresize(Some(resize.as_ref().unchecked_ref()));
    resize.forget();

    let keydown = Closure::wrap(Box::new(on_keydown) as Box<dyn Fn(web_sys::KeyboardEvent)>);
    window().set_onkeydown(Some(keydown.as_ref().unchecked_ref()));
    keydown.forget();

    let keyup = Closure::wrap(Box::new(on_keyup) as Box<dyn Fn(web_sys::KeyboardEvent)>);
    window().set_onkeyup(Some(keyup.as_ref().unchecked_ref()));
    keyup.forget();

    Ok(())
}

fn on_resize() {
    let window = window();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
    let mut state = HANDLE.lock().unwrap();
    state.viewport = Some(Viewport { width, height });
}

fn parse_key_event(e: web_sys::KeyboardEvent) -> (app::Key, u8) {
    let key = from_key_code(e.code());
    let mut mods: u8 = 0;
    if e.shift_key() {
        mods |= modifiers::SHIFT;
    }
    if e.ctrl_key() {
        mods |= modifiers::CTRL;
    }
    if e.alt_key() {
        mods |= modifiers::ALT;
    }
    if e.meta_key() {
        mods |= modifiers::META;
    }
    (key, mods)
}

fn on_keydown(e: web_sys::KeyboardEvent) {
    let mut state = HANDLE.lock().unwrap();
    let (key, mods) = parse_key_event(e);
    state.keyboard.on_keydown(key, mods);
}

fn on_keyup(e: web_sys::KeyboardEvent) {
    let mut state = HANDLE.lock().unwrap();
    let key = from_key_code(e.code());
    state.keyboard.on_keyup(key);
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
