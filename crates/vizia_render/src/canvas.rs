use anyrender::PaintScene;

/// This will wrap [PaintScene](anyrender::PaintScene)
pub struct Canvas<T: PaintScene> {
    paint: T,
}
