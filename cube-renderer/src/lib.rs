use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("The Cube is ready to be rendered.");
}

#[wasm_bindgen]
pub fn on_click() {
    log("The Cube is clicked.");
}
