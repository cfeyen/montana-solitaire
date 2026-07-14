mod app;

pub use app::{Message, Solitaire};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    app::start()
}
