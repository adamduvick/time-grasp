use leptos::html::Button;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static COLLAPSIBLE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Context shared between Collapsible components.
#[derive(Clone, Copy)]
struct CollapsibleContext {
    open: RwSignal<bool>,
    disabled: Signal<bool>,
    content_id: RwSignal<String>,
}

/// Root container for a collapsible section.
#[component]
pub fn CollapsibleRoot(
    /// Controlled open state.
    open: RwSignal<bool>,

    /// Whether the collapsible is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The collapsible content (trigger and content).
    children: ChildrenFn,
) -> impl IntoView {
    // Generate a unique ID for aria-controls
    let id = COLLAPSIBLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let content_id = RwSignal::new(format!("collapsible-content-{}", id));

    let ctx = CollapsibleContext {
        open,
        disabled,
        content_id,
    };

    provide_context(ctx);

    let state_attr = move || if open.get() { "open" } else { "closed" };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-collapsible-root=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
        >
            {children()}
        </div>
    }
}

/// Trigger button that toggles the collapsible open/closed state.
#[component]
pub fn CollapsibleTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the trigger element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The trigger content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<CollapsibleContext>()
        .expect("CollapsibleTrigger must be used within CollapsibleRoot");

    let on_click = move |_| {
        if !ctx.disabled.get() {
            ctx.open.update(|o| *o = !*o);
        }
    };

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            aria-expanded=move || ctx.open.get().to_string()
            aria-controls=move || ctx.content_id.get()
            disabled=move || ctx.disabled.get()
            data-radix-collapsible-trigger=""
            data-state=state_attr
            data-disabled=move || ctx.disabled.get().then_some("")
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Content that is shown/hidden based on the open state.
#[component]
pub fn CollapsibleContent(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the content element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The collapsible content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<CollapsibleContext>()
        .expect("CollapsibleContent must be used within CollapsibleRoot");

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };

    view! {
        <div
            node_ref=node_ref
            id=move || ctx.content_id.get()
            class=class
            style=style
            hidden=move || !ctx.open.get()
            data-radix-collapsible-content=""
            data-state=state_attr
            data-disabled=move || ctx.disabled.get().then_some("")
        >
            {children()}
        </div>
    }
}
