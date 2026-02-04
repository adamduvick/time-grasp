mod accordion;
mod alert_dialog;
mod arrow;
mod aspect_ratio;
mod avatar;
mod checkbox;
mod collapsible;
mod context_menu;
mod dialog;
mod dropdown;
mod form;
mod home;
mod hover_card;
mod label;
mod menu;
mod menubar;
mod navigation_menu;
mod otp_field;
mod password_toggle_field;
mod popover;
mod popper;
mod progress;
mod radio_group;
mod scroll_area;
mod select;
mod separator;
mod slider;
mod switch;
mod tabs;
mod toast;
mod toggle;
mod toggle_group;
mod toolbar;
mod tooltip;
mod visually_hidden;

pub use accordion::*;
pub use alert_dialog::*;
pub use arrow::*;
pub use aspect_ratio::*;
pub use avatar::*;
pub use checkbox::*;
pub use collapsible::*;
pub use context_menu::*;
pub use dialog::*;
pub use dropdown::*;
pub use form::*;
pub use home::*;
pub use hover_card::*;
pub use label::*;
pub use menu::*;
pub use menubar::*;
pub use navigation_menu::*;
pub use otp_field::*;
pub use password_toggle_field::*;
pub use popover::*;
pub use popper::*;
pub use progress::*;
pub use radio_group::*;
pub use scroll_area::*;
pub use select::*;
pub use separator::*;
pub use slider::*;
pub use switch::*;
pub use tabs::*;
pub use toast::*;
pub use toggle::*;
pub use toggle_group::*;
pub use toolbar::*;
pub use tooltip::*;
pub use visually_hidden::*;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Primitive {
    pub path: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    #[serde(default)]
    pub implemented: bool,
}

const PRIMITIVES_JSON: &str = include_str!("../primitives.json");

pub fn primitives() -> Vec<Primitive> {
    serde_json::from_str(PRIMITIVES_JSON).expect("primitives.json is valid")
}
