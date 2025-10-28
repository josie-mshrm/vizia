pub use skia_safe::Data;
pub use skia_safe::Image;

pub enum ImageOrSvg {
    Svg(skia_safe::svg::Dom),
    Image(skia_safe::Image),
}
