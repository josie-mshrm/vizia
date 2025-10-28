use vizia_style::{Angle, Scale, Transform, Translate};

use crate::layout::BoundingBox;

/// Trait for converting a transform definition into a `Matrix`.
pub(crate) trait IntoTransform {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Affine;
}

impl IntoTransform for Translate {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Affine {
        let tx = self.x.to_pixels(bounds.w, scale_factor);
        let ty = self.y.to_pixels(bounds.h, scale_factor);

        Affine::translate((tx, ty))
    }
}

impl IntoTransform for Scale {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Affine {
        let sx = self.x.to_factor();
        let sy = self.y.to_factor();

        Affine::scale((sx, sy))
    }
}

impl IntoTransform for Angle {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Affine {
        let r = self.to_radians();

        Affine::rotate_rad(r)
    }
}

impl IntoTransform for Vec<Transform> {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Affine {
        let mut result = Affine::IDENTITY;
        for transform in self.iter() {
            let t = match transform {
                Transform::Translate(translate) => {
                    let tx = translate.0.to_pixels(bounds.w, scale_factor);
                    let ty = translate.1.to_pixels(bounds.h, scale_factor);

                    Affine::translate((tx, ty))
                }

                Transform::TranslateX(x) => {
                    let tx = x.to_pixels(bounds.w, scale_factor);

                    Affine::translate((tx, 0.0))
                }

                Transform::TranslateY(y) => {
                    let ty = y.to_pixels(bounds.h, scale_factor);

                    Affine::translate((0.0, ty))
                }

                Transform::Scale(scale) => {
                    let sx = scale.0.to_factor();
                    let sy = scale.1.to_factor();

                    Affine::scale((sx, sy))
                }

                Transform::ScaleX(x) => {
                    let sx = x.to_factor();

                    Affine::scale((sx, 1.0))
                }

                Transform::ScaleY(y) => {
                    let sy = y.to_factor();

                    Affine::scale_non_uniform(1.0, sy)
                }

                Transform::Rotate(angle) => Affine::rotate_rad(angle.to_radians()),

                Transform::Skew(x, y) => {
                    let cx = x.to_radians().tan();
                    let cy = y.to_radians().tan();

                    Affine::skew(cx, cy)
                }

                Transform::SkewX(angle) => {
                    let cx = angle.to_radians().tan();

                    Affine::skew(cx, 0.0)
                }

                Transform::SkewY(angle) => {
                    let cy = angle.to_radians().tan();

                    Affine::skew(0.0, cy)
                }

                Transform::Matrix(matrix) => {
                    Affine::new(&[matrix.a, matrix.c, matrix.e, matrix.b, matrix.d, matrix.f])
                }
            };

            result = result * t;
        }

        result
    }
}
