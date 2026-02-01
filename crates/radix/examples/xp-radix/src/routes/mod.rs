mod accordion;
mod alert_dialog;
mod aspect_ratio;
mod avatar;
mod checkbox;
mod collapsible;
mod context_menu;
mod dialog;
mod dropdown;
mod home;
mod hover_card;
mod menubar;
mod navigation_menu;
mod popover;
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

pub use accordion::*;
pub use alert_dialog::*;
pub use aspect_ratio::*;
pub use avatar::*;
pub use checkbox::*;
pub use collapsible::*;
pub use context_menu::*;
pub use dialog::*;
pub use dropdown::*;
pub use home::*;
pub use hover_card::*;
pub use menubar::*;
pub use navigation_menu::*;
pub use popover::*;
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

pub fn primitives() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "/accordion",
            "Accordion",
            "Collapsible sections with single/multiple modes",
        ),
        (
            "/alert-dialog",
            "AlertDialog",
            "Modal for confirmations requiring explicit action",
        ),
        (
            "/aspect-ratio",
            "AspectRatio",
            "Maintains consistent width/height ratio",
        ),
        (
            "/avatar",
            "Avatar",
            "User avatar with image and fallback support",
        ),
        (
            "/checkbox",
            "Checkbox",
            "Tri-state checkbox with indeterminate support",
        ),
        (
            "/collapsible",
            "Collapsible",
            "Simple expand/collapse container",
        ),
        (
            "/context-menu",
            "ContextMenu",
            "Right-click menu with submenus",
        ),
        (
            "/dialog",
            "Dialog",
            "Modal overlay pattern with focus trap and portal",
        ),
        (
            "/dropdown",
            "DropdownMenu",
            "Menu triggered by button with keyboard navigation",
        ),
        (
            "/hover-card",
            "HoverCard",
            "Rich content on hover with open/close delays",
        ),
        (
            "/menubar",
            "Menubar",
            "Application menu bar with keyboard navigation",
        ),
        (
            "/navigation-menu",
            "NavigationMenu",
            "Site navigation with dropdown submenus",
        ),
        (
            "/popover",
            "Popover",
            "Floating content anchored to a trigger element",
        ),
        (
            "/progress",
            "Progress",
            "Progress indicator with determinate/indeterminate states",
        ),
        (
            "/radio-group",
            "RadioGroup",
            "Single selection from multiple options",
        ),
        (
            "/scroll-area",
            "ScrollArea",
            "Custom scrollbars with consistent cross-browser styling",
        ),
        ("/select", "Select", "Custom dropdown select with typeahead"),
        (
            "/separator",
            "Separator",
            "Visual divider between content sections",
        ),
        (
            "/slider",
            "Slider",
            "Range input with single or multiple thumbs",
        ),
        ("/switch", "Switch", "Toggle switch for boolean settings"),
        (
            "/tabs",
            "Tabs",
            "Tabbed content panels with keyboard navigation",
        ),
        (
            "/toast",
            "Toast",
            "Temporary notifications with auto-dismiss",
        ),
        ("/toggle", "Toggle", "On/off button with controlled state"),
        (
            "/toggle-group",
            "ToggleGroup",
            "Group of toggles with single/multiple selection",
        ),
        (
            "/toolbar",
            "Toolbar",
            "Toolbar container with toggle groups and buttons",
        ),
        (
            "/tooltip",
            "Tooltip",
            "Hover/focus hints with delay and positioning",
        ),
    ]
}
