use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    /// The desired aspect ratio (width/height). Default is 1.0 (square).
    #[prop(default = 1.0.into(), into)]
    ratio: Signal<f64>,

    /// Reference to the outer container element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The content to render within the aspect ratio container.
    children: ChildrenFn,
) -> impl IntoView {
    let padding_bottom = move || {
        let r = ratio.get();
        if r <= 0.0 {
            "100%".to_string()
        } else {
            format!("{}%", (1.0 / r) * 100.0)
        }
    };

    view! {
        <div
            node_ref=node_ref
            style:position="relative"
            style:width="100%"
            style:padding-bottom=padding_bottom
            data-radix-aspect-ratio=""
        >
            <div
                style:position="absolute"
                style:top="0"
                style:right="0"
                style:bottom="0"
                style:left="0"
            >
                {children()}
            </div>
        </div>
    }
}
