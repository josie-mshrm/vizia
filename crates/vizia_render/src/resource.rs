pub enum ImageOrSvg {
    Svg(skia_safe::svg::Dom),
    Image(skia_safe::Image),
}
