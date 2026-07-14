#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod app;

pub use app::{Message, Solitaire};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    app::start()
        .map_err(|err| JsValue::from_str(&err.to_string()))
}