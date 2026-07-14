#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod app;

pub use app::{Message, Solitaire};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use app::FONT;

    console_error_panic_hook::set_once();
    iced::application(Solitaire::default, Solitaire::update, Solitaire::view)
        .font(include_bytes!("../NotoSerif-VariableFont_wdth,wght.ttf"))
        .default_font(FONT)
        .run()
        .map_err(|err| JsValue::from_str(&err.to_string()))
}