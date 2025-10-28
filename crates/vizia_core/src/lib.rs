//! Vizia

#![allow(clippy::type_complexity)]
// To allow a match syntax in event handlers with one event variant
#![allow(clippy::single_match)]
// To allow enum names with the same prefix
#![allow(clippy::enum_variant_names)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::uninlined_format_args)]
#![allow(mismatched_lifetime_syntaxes)]
// #![warn(missing_docs)]

extern crate self as vizia;

pub(crate) mod accessibility;

pub mod animation;
pub mod application;
pub mod binding;
#[doc(hidden)]
pub(crate) mod cache;
pub mod context;
pub mod environment;
pub mod events;
pub mod input;
pub mod layout;
pub mod localization;
pub mod model;
pub mod modifiers;
pub mod recoil;
pub mod resource;
pub mod style;
pub(crate) mod systems;
#[doc(hidden)]
pub mod tree;
pub mod view;
pub mod views;
pub mod window;

mod storage;

/// A collection of built-in SVG icons.
pub mod icons;

#[doc(hidden)]
pub mod backend {
    pub use super::accessibility::IntoNode;
    pub use super::context::backend::BackendContext;
    pub use vizia_window::WindowDescription;
}

/// Members which we recommend you wildcard-import.
#[doc(hidden)]
pub mod prelude {
    pub use crate::application::App;
    pub use vizia_render::prelude::*;

    pub use super::binding::{
        Binding, Data, Index, Lens, LensExt, LensValue, Map, MapRef, Res, ResGet, StaticLens, Then,
        UnwrapLens, Wrapper,
    };
    pub use super::recoil::*;

    pub use super::impl_res_simple;

    pub use crate::model::Model;

    pub use super::animation::{Animation, AnimationBuilder, KeyframeBuilder};
    pub use super::context::{
        AccessContext, AccessNode, Context, ContextProxy, DataContext, DrawContext, EmitContext,
        EventContext, ProxyEmitError, WindowState,
    };
    pub use super::environment::{AppTheme, Environment, EnvironmentEvent, ThemeMode};
    pub use super::events::{Event, Propagation, Timer, TimerAction};
    pub use super::input::{Keymap, KeymapEntry, KeymapEvent};
    pub use super::localization::{Localized, ToStringLocalized};
    pub use super::modifiers::{
        AbilityModifiers, AccessibilityModifiers, ActionModifiers, LayoutModifiers,
        LinearGradientBuilder, ShadowBuilder, StyleModifiers, TextModifiers,
    };
    pub use super::resource::{ImageId, ImageRetentionPolicy};
    pub use super::view::{Handle, View};
    pub use super::views::*;
    pub use super::window::{DropData, WindowEvent};
    pub use accesskit::{Action, Live, Role};
    pub use vizia_derive::{Data, Lens};
    pub use vizia_id::GenerationalId;
    pub use vizia_input::{Code, Key, KeyChord, Modifiers, MouseButton, MouseButtonState};
    pub use vizia_storage::{Tree, TreeExt};
    pub use vizia_window::{Anchor, AnchorTarget, WindowButtons, WindowPosition, WindowSize};

    pub use super::style::*;

    pub use morphorm::Units::*;
    pub use morphorm::{LayoutType, PositionType, Units};
    pub use unic_langid::{langid, LanguageIdentifier};
    pub use web_time::{Duration, Instant};
}
