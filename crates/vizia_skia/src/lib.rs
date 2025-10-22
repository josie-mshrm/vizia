pub mod layout;
pub mod text;
pub mod views;

pub use skia_safe::Canvas;

/// Contains types and functions used for custom drawing within views. This is a re-export of [skia-safe](https://github.com/rust-skia/rust-skia).
pub mod vg {
    pub use skia_safe::*;
}
