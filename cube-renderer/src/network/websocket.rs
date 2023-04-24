use std::{collections::HashMap, sync::Mutex};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MessageEvent;

use crate::{
    console,
    utils::{Message, Serializable},
};

lazy_static::lazy_static! {
    static ref POOLS: Mutex<HashMap<&'static str, Vec<Message>>> = Mutex::new(HashMap::new());
}

pub struct WebSocket {
    inner: web_sys::WebSocket,
    pool: &'static str,
}

macro_rules! get_pool {
    ($lock:expr, $pool:expr) => {{
        let pool = if let Some(pool) = $lock.get_mut($pool) {
            pool
        } else {
            $lock.insert($pool, Vec::new());
            $lock.get_mut($pool).unwrap()
        };
        pool
    }};
}

impl WebSocket {
    pub fn new(url: &'static str, pool: &'static str) -> Self {
        let inner = web_sys::WebSocket::new(url).unwrap();

        let on_open = Closure::wrap(Box::new(move || {
            console::log!("WebSocket opened");
        }) as Box<dyn FnMut()>);
        inner.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();

        let mut this = Self { inner, pool };

        this.setup_listeners();
        this
    }

    fn read_message(pool: &'static str, e: MessageEvent) {
        let on_load_cb = move |buffer: wasm_bindgen::JsValue| {
            let data = js_sys::Uint8Array::new(&buffer);
            let data = data.to_vec();
            let bytes = data.as_slice();
            let message = Message::from_bytes(bytes);
            console::log!("WebSocket message: {:?}", message);
            match message {
                Ok(message) => {
                    let mut pools = POOLS.lock().unwrap();
                    let pool = get_pool!(pools, pool);
                    pool.push(message)
                }
                Err(e) => console::error!("WebSocket message error: {:?}", e),
            }
        };

        if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            let on_load = Closure::wrap(Box::new(on_load_cb) as Box<dyn FnMut(_)>);
            let _ = blob.array_buffer().then(&on_load);
            on_load.forget();
        } else if let Ok(buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            on_load_cb(buffer.into());
        }
    }

    fn setup_listeners(&mut self) {
        let pool = self.pool;
        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            Self::read_message(pool, e);
        }) as Box<dyn FnMut(MessageEvent)>);
        self.inner
            .set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();

        let on_close = Closure::wrap(Box::new(move || {
            console::log!("WebSocket closed");
        }) as Box<dyn FnMut()>);
        self.inner
            .set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();
    }

    pub fn poll(&mut self) -> impl Iterator<Item = Message> {
        let mut pools = POOLS.lock().unwrap();
        let pool = get_pool!(pools, self.pool);

        pool.drain(..).collect::<Vec<_>>().into_iter()
    }
}
