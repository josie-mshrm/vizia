#[doc(hidden)]
pub mod entity;
pub mod layout;
pub mod resource;
pub mod text;
pub mod util;

pub use vizia_style as style;

/// Members which we recommend you wildcard-import.
#[doc(hidden)]
pub mod prelude {
    // pub use super::binding::{
    //     Binding, Data, Index, Lens, LensExt, LensValue, Map, MapRef, Res, ResGet, StaticLens, Then,
    //     UnwrapLens, Wrapper,
    // };
    // pub use super::recoil::*;

    // pub use super::impl_res_simple;

    // pub use crate::model::Model;

    // pub use super::animation::{Animation, AnimationBuilder, KeyframeBuilder};
    pub use super::entity::Entity;
    // pub use super::environment::{AppTheme, Environment, EnvironmentEvent, ThemeMode};
    // pub use super::events::{Event, Propagation, Timer, TimerAction};
    pub use super::include_style;
    // pub use super::input::{Keymap, KeymapEntry, KeymapEvent};
    // pub use super::localization::{Localized, ToStringLocalized};
    // pub use super::modifiers::{
    //     AbilityModifiers, AccessibilityModifiers, ActionModifiers, LayoutModifiers,
    //     LinearGradientBuilder, ShadowBuilder, StyleModifiers, TextModifiers,
    // };
    pub use super::layout::{BoundingBox, GeoChanged};
    // pub use super::resource::{ImageId, ImageRetentionPolicy};
    pub use super::resource::ImageOrSvg;
    pub use super::util::{IntoCssStr, CSS};
    // pub use super::view::{Handle, View};
    // pub use super::views::*;
    // pub use super::window::{DropData, WindowEvent};
    pub use accesskit::{Action, Live, Role};
    // pub use skia_safe::Canvas;
    pub use vizia_derive::{Data, Lens};
    pub use vizia_id::GenerationalId;
    pub use vizia_input::{Code, Key, KeyChord, Modifiers, MouseButton, MouseButtonState};
    pub use vizia_storage::{Tree, TreeExt};
    pub use vizia_window::{Anchor, AnchorTarget, WindowButtons, WindowPosition, WindowSize};

    pub use morphorm::Units::*;
    pub use morphorm::{LayoutType, PositionType, Units};
    pub use unic_langid::{langid, LanguageIdentifier};
    pub use web_time::{Duration, Instant};
}
