use leptos::html::Button;
use leptos::prelude::*;

/// A two-state button that can be on or off. Uses aria-pressed for accessibility.
#[component]
pub fn ToggleRoot(
    /// Controlled pressed state of the toggle.
    pressed: RwSignal<bool>,

    /// Whether the toggle is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The toggle content (text, icon, etc.).
    children: ChildrenFn,
) -> impl IntoView {
    let on_click = move |_| {
        if !disabled.get() {
            pressed.update(|p| *p = !*p);
        }
    };

    let state_attr = move || if pressed.get() { "on" } else { "off" };

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            aria-label=aria_label
            aria-pressed=move || pressed.get().to_string()
            disabled=move || disabled.get()
            data-radix-toggle-root=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
        >
            {children()}
        </button>
    }
}
