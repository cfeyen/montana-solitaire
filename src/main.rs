
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod app;

pub use app::{Message, Solitaire};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    iced::run(Solitaire::update, Solitaire::view).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    iced::run(Solitaire::update, Solitaire::view)
}
