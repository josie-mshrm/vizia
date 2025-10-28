use std::ops::{Index, IndexMut};

use kurbo::Affine as KurboAffine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine([f32; 6]);

impl Affine {
    pub const IDENTITY: Affine = Affine::scale(1.0);

    /// Creates a new Identity Affine
    pub const fn new(value: [f32; 6]) -> Self {
        Affine(value)
    }

    pub const fn scale(scale: f32) -> Self {
        Affine([scale, 0.0, 0.0, scale, 0.0, 0.0])
    }

    pub fn new_from(value: Self) -> Self {
        Affine(value.0)
    }

    pub fn to_kurbo(self) -> KurboAffine {
        KurboAffine::from(self)
    }
}

impl From<Affine> for KurboAffine {
    fn from(value: Affine) -> Self {
        let value: [f64; 6] = value
            .0
            .iter()
            .map(|n| *n as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .expect("[f64] is of known size");

        KurboAffine::new(value)
    }
}

impl From<&Affine> for KurboAffine {
    fn from(value: &Affine) -> Self {
        let value: [f64; 6] = value
            .0
            .iter()
            .map(|n| *n as f64)
            .collect::<Vec<f64>>()
            .try_into()
            .expect("[f64] is of known size");

        KurboAffine::new(value)
    }
}

impl From<KurboAffine> for Affine {
    fn from(value: KurboAffine) -> Self {
        Affine::new(
            value
                .as_coeffs()
                .iter()
                .map(|n| *n as f32)
                .collect::<Vec<f32>>()
                .try_into()
                .expect("[f32] is of known size"),
        )
    }
}

impl Index<usize> for Affine {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Affine {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
