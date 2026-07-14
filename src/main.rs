mod app;

pub use app::{Message, Solitaire};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> iced::Result {
    use app::FONT;

    iced::application(Solitaire::default, Solitaire::update, Solitaire::view)
        .font(include_bytes!("../NotoSerif-VariableFont_wdth,wght.ttf"))
        .default_font(FONT)
        .run()
}
