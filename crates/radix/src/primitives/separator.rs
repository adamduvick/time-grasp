use leptos::html::Div;
use leptos::prelude::*;

/// Orientation for the separator
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A visual or semantic separator between content.
///
/// Based on [Radix UI Separator](https://www.radix-ui.com/primitives/docs/components/separator).
#[component]
pub fn Separator(
    /// The orientation of the separator.
    #[prop(default = SeparatorOrientation::Horizontal.into(), into)]
    orientation: Signal<SeparatorOrientation>,

    /// When true, the separator is purely decorative and has no semantic meaning.
    /// When false (default), it acts as a semantic separator with role="separator".
    #[prop(default = false.into(), into)]
    decorative: Signal<bool>,

    /// Reference to the separator element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Optional class name.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let orientation_attr = move || match orientation.get() {
        SeparatorOrientation::Horizontal => "horizontal",
        SeparatorOrientation::Vertical => "vertical",
    };

    let aria_orientation = move || {
        if decorative.get() {
            None
        } else {
            match orientation.get() {
                // aria-orientation defaults to horizontal, so we only set for vertical
                SeparatorOrientation::Horizontal => None,
                SeparatorOrientation::Vertical => Some("vertical"),
            }
        }
    };

    let role = move || {
        if decorative.get() {
            "none"
        } else {
            "separator"
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class.unwrap_or_default()
            role=role
            aria-orientation=aria_orientation
            data-orientation=orientation_attr
            data-radix-separator=""
        />
    }
}
