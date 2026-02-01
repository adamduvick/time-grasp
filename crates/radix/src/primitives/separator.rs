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
    #[prop(default = SeparatorOrientation::default().into(), into)]
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
    let ctx = SeparatorContext::new(orientation, decorative);
    view! {
        <div
            node_ref=node_ref
            class=class.unwrap_or_default()
            role=ctx.role
            aria-orientation=ctx.aria_orientation
            data-orientation=ctx.orientation_attr
            data-radix-separator=""
        />
    }
}

struct SeparatorContext {
    orientation_attr: Memo<&'static str>,
    aria_orientation: Memo<Option<&'static str>>,
    role: Memo<&'static str>,
}

impl SeparatorContext {
    fn new(orientation: Signal<SeparatorOrientation>, decorative: Signal<bool>) -> Self {
        let orientation_attr = Memo::new(move |_| match orientation.get() {
            SeparatorOrientation::Horizontal => "horizontal",
            SeparatorOrientation::Vertical => "vertical",
        });

        let aria_orientation = Memo::new(move |_| {
            if decorative.get() {
                None
            } else {
                match orientation.get() {
                    // aria-orientation defaults to horizontal, so we only set for vertical
                    SeparatorOrientation::Horizontal => None,
                    SeparatorOrientation::Vertical => Some("vertical"),
                }
            }
        });

        let role = Memo::new(move |_| {
            if decorative.get() {
                "none"
            } else {
                "separator"
            }
        });

        Self {
            orientation_attr,
            aria_orientation,
            role,
        }
    }
}
