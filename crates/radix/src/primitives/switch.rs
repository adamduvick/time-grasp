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

    // Consolidated style to stay under attribute limit
    let style = move || {
        let bg = if checked.get() { "#3b82f6" } else { "#e0e0e0" };
        format!(
            "position:relative;display:inline-flex;align-items:center;\
             width:44px;height:24px;padding:2px;border:none;border-radius:9999px;\
             background:{};cursor:{};outline:none;transition:background 0.2s",
            bg,
            if disabled.get() { "not-allowed" } else { "pointer" }
        )
    };

    view! {
        <button
            node_ref=node_ref
            type="button"
            role="switch"
            aria-checked=move || checked.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            disabled=move || disabled.get()
            style=style
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
    /// Reference to the thumb element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,
) -> impl IntoView {
    let ctx = use_context::<SwitchContext>().expect("SwitchThumb must be used within SwitchRoot");

    let state_attr = move || if ctx.checked.get() { "checked" } else { "unchecked" };

    // Consolidated style - thumb slides left/right based on checked state
    let style = move || {
        let translate_x = if ctx.checked.get() { "20px" } else { "0px" };
        format!(
            "display:block;width:20px;height:20px;background:white;border-radius:50%;\
             box-shadow:0 1px 3px rgba(0,0,0,0.2);transition:transform 0.2s;\
             transform:translateX({})",
            translate_x
        )
    };

    view! {
        <span
            node_ref=node_ref
            style=style
            data-radix-switch-thumb=""
            data-state=state_attr
            data-disabled=move || ctx.disabled.get().then_some("")
        />
    }
}
