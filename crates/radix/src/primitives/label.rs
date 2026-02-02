use leptos::html::Label as HtmlLabel;
use leptos::prelude::*;

/// An accessible label for form controls.
///
/// Based on [Radix UI Label](https://www.radix-ui.com/primitives/docs/components/label).
#[component]
pub fn Label(
    /// Associates the label with a form control by ID.
    #[prop(optional, into)]
    html_for: Option<String>,

    /// Optional class name.
    #[prop(optional, into)]
    class: Option<String>,

    /// Optional inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the label element.
    #[prop(optional)]
    node_ref: NodeRef<HtmlLabel>,

    /// Label content.
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <label
            node_ref=node_ref
            for=html_for
            class=class
            style=style
            data-radix-label=""
        >
            {children()}
        </label>
    }
}
