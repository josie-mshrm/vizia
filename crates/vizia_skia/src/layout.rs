use vizia_core::layout::BoundingBox;

impl From<BoundingBox> for skia_safe::Rect {
    fn from(bb: BoundingBox) -> Self {
        skia_safe::Rect { left: bb.left(), top: bb.top(), right: bb.right(), bottom: bb.bottom() }
    }
}

impl From<skia_safe::Rect> for BoundingBox {
    fn from(bb: Rect) -> Self {
        BoundingBox { x: bb.left(), y: bb.top(), w: bb.width(), h: bb.height() }
    }
}
