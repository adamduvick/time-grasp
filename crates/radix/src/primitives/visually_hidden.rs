use leptos::prelude::*;

/// The CSS styles that hide content visually while keeping it accessible.
const VISUALLY_HIDDEN_STYLES: &str = "\
    position: absolute; \
    border: 0; \
    width: 1px; \
    height: 1px; \
    padding: 0; \
    margin: -1px; \
    overflow: hidden; \
    clip: rect(0, 0, 0, 0); \
    white-space: nowrap; \
    word-wrap: normal;\
";

/// Hides content visually while keeping it accessible to screen readers.
///
/// Uses CSS technique that maintains accessibility - unlike `display: none`
/// or `visibility: hidden`, this keeps content in the accessibility tree.
///
/// Use cases:
/// - Icon-only buttons that need accessible names
/// - Additional context for screen reader users
/// - Skip navigation links
/// - Visually redundant table headers
/// - Form instructions that clutter visual design
#[component]
pub fn VisuallyHidden(
    /// Additional CSS class name(s).
    #[prop(optional, into)]
    class: Option<String>,

    /// Additional inline styles (merged with hiding styles).
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the span element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,

    /// The content to hide visually.
    children: ChildrenFn,
) -> impl IntoView {
    let combined_style = match style {
        Some(s) => format!("{} {}", VISUALLY_HIDDEN_STYLES, s),
        None => VISUALLY_HIDDEN_STYLES.to_string(),
    };

    view! {
        <span
            node_ref=node_ref
            class=class
            style=combined_style
            data-radix-visually-hidden=""
        >
            {children()}
        </span>
    }
}
