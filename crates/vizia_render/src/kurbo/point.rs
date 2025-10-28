use kurbo::Point as KurboPoint;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl From<Point> for KurboPoint {
    fn from(value: Point) -> Self {
        KurboPoint::new(value.x as f64, value.y as f64)
    }
}

impl From<&Point> for KurboPoint {
    fn from(value: &Point) -> Self {
        KurboPoint::new(value.x as f64, value.y as f64)
    }
}

impl From<KurboPoint> for Point {
    fn from(value: KurboPoint) -> Self {
        Point { x: value.x as f32, y: value.y as f32 }
    }
}
