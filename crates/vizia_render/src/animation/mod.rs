mod animation_state;
pub(crate) use animation_state::{AnimationState, Keyframe};

mod interpolator;
pub(crate) use interpolator::Interpolator;

mod timing_function;
pub(crate) use timing_function::TimingFunction;

mod animation_builder;
pub use animation_builder::*;
use vizia_id::*;

/// An ID used to reference style animations stored in the style store.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animation(u64);

impl_generational_id!(Animation);
