use leptos::prelude::*;

/// Arrow primitive - a low-level SVG arrow shape for composition.
///
/// This is NOT an accessibility primitive - it's a composition helper
/// used internally by Tooltip, Popover, HoverCard, etc.
///
/// The arrow points DOWN by default (tip at bottom center).
/// Rotate as needed for other orientations.
///
/// Key details matching React Radix:
/// - viewBox is always "0 0 30 10" regardless of width/height
/// - preserveAspectRatio="none" allows independent scaling
/// - Default polygon: "0,0 30,0 15,10" (flat top, tip at bottom center)
#[component]
pub fn Arrow(
    /// Width of the arrow in pixels.
    #[prop(default = 10)]
    width: u32,

    /// Height of the arrow in pixels.
    #[prop(default = 5)]
    height: u32,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Custom arrow content (replaces default polygon).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <svg
            class=class
            style=style
            width=width
            height=height
            viewBox="0 0 30 10"
            preserveAspectRatio="none"
            data-radix-arrow=""
        >
            {match children {
                Some(children) => children().into_any(),
                // Default polygon: flat top edge, tip at bottom center
                // Matches React Radix: "0,0 30,0 15,10"
                None => view! { <polygon points="0,0 30,0 15,10" /> }.into_any(),
            }}
        </svg>
    }
}
