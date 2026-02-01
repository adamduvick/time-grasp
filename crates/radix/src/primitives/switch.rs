use leptos::html::Button;
use leptos::prelude::*;

/// Context shared between Switch components
#[derive(Clone, Copy)]
struct SwitchContext {
    checked: RwSignal<bool>,
    disabled: Signal<bool>,
}

/// Root container for the switch. Renders as a button and manages checked state.
#[component]
pub fn SwitchRoot(
    /// Controlled checked state of the switch.
    checked: RwSignal<bool>,

    /// Whether the switch is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The switch content (typically SwitchThumb).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = SwitchContext { checked, disabled };

    provide_context(ctx);

    let on_click = move |_| {
        if !disabled.get() {
            checked.update(|c| *c = !*c);
            // Ensure focus is set for keyboard navigation
            if let Some(el) = node_ref.get() {
                _ = el.focus();
            }
        }
    };

    let state_attr = move || if checked.get() { "checked" } else { "unchecked" };

    view! {
        <button
            node_ref=node_ref
            type="button"
            role="switch"
            class=class
            style=style
            aria-checked=move || checked.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            disabled=move || disabled.get()
            data-radix-switch-root=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Visual thumb indicator that slides between positions based on checked state.
#[component]
pub fn SwitchThumb(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the thumb element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the thumb element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,
) -> impl IntoView {
    let ctx = use_context::<SwitchContext>().expect("SwitchThumb must be used within SwitchRoot");

    let state_attr = move || if ctx.checked.get() { "checked" } else { "unchecked" };

    view! {
        <span
            node_ref=node_ref
            class=class
            style=style
            data-radix-switch-thumb=""
            data-state=state_attr
            data-disabled=move || ctx.disabled.get().then_some("")
        />
    }
}
