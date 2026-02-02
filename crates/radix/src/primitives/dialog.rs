use leptos::html::Button;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

static DIALOG_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Global registry of dialog open signals, keyed by dialog ID
// This allows DialogClose to find the correct signal even in nested scenarios
thread_local! {
    static DIALOG_REGISTRY: std::cell::RefCell<std::collections::HashMap<usize, RwSignal<bool>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn register_dialog(id: usize, open: RwSignal<bool>) {
    DIALOG_REGISTRY.with(|r| {
        r.borrow_mut().insert(id, open);
    });
}

fn unregister_dialog(id: usize) {
    DIALOG_REGISTRY.with(|r| {
        r.borrow_mut().remove(&id);
    });
}

fn get_dialog_signal(id: usize) -> Option<RwSignal<bool>> {
    DIALOG_REGISTRY.with(|r| r.borrow().get(&id).copied())
}

/// Context shared between Dialog components.
#[derive(Clone, Copy)]
struct DialogContext {
    open: RwSignal<bool>,
    title_id: StoredValue<String>,
    description_id: StoredValue<String>,
    dialog_id: usize,
}

/// Root container for a dialog.
#[component]
pub fn DialogRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Callback when open state changes.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,

    /// The dialog parts.
    children: ChildrenFn,
) -> impl IntoView {
    let id = DIALOG_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let title_id = StoredValue::new(format!("dialog-title-{}", id));
    let description_id = StoredValue::new(format!("dialog-description-{}", id));

    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));

    // Register this dialog's signal in the global registry
    register_dialog(id, open_signal);

    // Unregister when component is cleaned up
    on_cleanup(move || {
        unregister_dialog(id);
    });

    // Notify on_open_change when open state changes
    if let Some(callback) = on_open_change {
        Effect::new(move |_| {
            callback.run(open_signal.get());
        });
    }

    let ctx = DialogContext {
        open: open_signal,
        title_id,
        description_id,
        dialog_id: id,
    };

    provide_context(ctx);

    children()
}

/// Trigger button that opens the dialog.
#[component]
pub fn DialogTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The trigger content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogTrigger must be used within DialogRoot");

    let on_click = move |_: web_sys::MouseEvent| {
        ctx.open.set(true);
    };

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            data-radix-dialog-trigger=""
            data-state=state_attr
            data-dialog-id=ctx.dialog_id.to_string()
            aria-haspopup="dialog"
            aria-expanded=move || ctx.open.get().to_string()
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Portal container - renders children to document.body when dialog is open.
#[component]
pub fn DialogPortal(
    /// The portal content (overlay and content).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogPortal must be used within DialogRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <Portal>
                <DialogPortalInner ctx=ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn DialogPortalInner(ctx: DialogContext, children: StoredValue<ChildrenFn>) -> impl IntoView {
    // Re-provide context inside the portal
    provide_context(ctx);

    children.with_value(|c| c())
}

/// Overlay backdrop behind the dialog.
#[component]
pub fn DialogOverlay(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the overlay element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogOverlay must be used within DialogRoot");

    // Click on overlay closes the dialog
    let on_click = move |_: web_sys::MouseEvent| {
        ctx.open.set(false);
    };

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-dialog-overlay=""
            data-state=state_attr
            data-dialog-id=ctx.dialog_id.to_string()
            on:click=on_click
        />
    }
}

/// The dialog content container.
#[component]
pub fn DialogContent(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogContent must be used within DialogRoot");

    let content_ref = node_ref;
    let dialog_id = ctx.dialog_id;

    // Store the trigger selector for focus restoration when dialog closes
    let trigger_selector = format!(
        "[data-radix-dialog-trigger][data-dialog-id=\"{}\"]",
        dialog_id
    );

    // Focus management - focus first focusable element when mounted
    Effect::new(move |_| {
        if let Some(el) = content_ref.get() {
            let html_el: &web_sys::HtmlElement = &el;

            // Find first focusable element
            if let Ok(Some(first)) = html_el.query_selector(
                "button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"])"
            ) {
                if let Ok(first_el) = first.dyn_into::<web_sys::HtmlElement>() {
                    let _ = first_el.focus();
                    return;
                }
            }

            // Fallback: focus the content itself
            let _ = html_el.focus();
        }
    });

    // Restore focus when dialog closes
    let trigger_selector_cleanup = trigger_selector.clone();
    on_cleanup(move || {
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector(&trigger_selector_cleanup).ok().flatten())
        {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                let _ = html_el.focus();
            }
        }
    });

    // Set up document-level keydown listener for Escape (capture phase)
    // This ensures we catch Escape even when focus is in an input
    Effect::new(move |_| {
        let document = web_sys::window().and_then(|w| w.document());
        let Some(doc) = document else { return };

        let handler =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() != "Escape" {
                    return;
                }

                // Check if this dialog's content exists
                let selector = format!(
                    "[data-radix-dialog-content][data-dialog-id=\"{}\"]",
                    dialog_id
                );
                let Some(content) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.query_selector(&selector).ok().flatten())
                else {
                    return;
                };

                let active_element = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.active_element());

                let is_focus_inside_content = active_element
                    .as_ref()
                    .map(|active| content.contains(Some(active)) || content == *active)
                    .unwrap_or(false);

                let is_focus_on_body = active_element
                    .as_ref()
                    .map(|el| el.tag_name().to_uppercase() == "BODY")
                    .unwrap_or(true);

                // Determine if we should handle this Escape
                let should_handle = if is_focus_inside_content {
                    true
                } else if is_focus_on_body {
                    // Focus is on body (nothing specific focused).
                    // Only handle if this is the topmost (last) open dialog.
                    let all_dialogs = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.query_selector_all("[data-radix-dialog-content]").ok());

                    match all_dialogs {
                        Some(list) if list.length() > 0 => {
                            // Get the last dialog content in DOM (topmost due to portal ordering)
                            list.get(list.length() - 1)
                                .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
                                .map(|el| el == content)
                                .unwrap_or(false)
                        }
                        _ => true,
                    }
                } else {
                    false
                };

                if !should_handle {
                    return;
                }

                // If focus is on an input-type element, just blur it (don't close dialog)
                if let Some(ref active) = active_element {
                    let tag_name = active.tag_name().to_uppercase();
                    if tag_name == "INPUT" || tag_name == "TEXTAREA" || tag_name == "SELECT" {
                        ev.prevent_default();
                        ev.stop_propagation();
                        // Blur the input - focus will go to body, second Escape will close
                        if let Ok(html_active) = active.clone().dyn_into::<web_sys::HtmlElement>()
                        {
                            let _ = html_active.blur();
                        }
                        return;
                    }
                }

                // Close the dialog
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(signal) = get_dialog_signal(dialog_id) {
                    signal.set(false);
                }
            });

        let _ = doc.add_event_listener_with_callback_and_bool(
            "keydown",
            handler.as_ref().unchecked_ref(),
            true, // capture phase
        );

        // Leak the handler - it's safe in WASM because:
        // 1. The handler checks if active element is inside THIS dialog's content
        // 2. When dialog closes, its content is removed from DOM so the check fails
        // 3. WASM is single-threaded so there are no race conditions
        handler.forget();
    });

    // Handle Tab key for focus trap
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() != "Tab" {
            return;
        }

        let Some(content_el) = content_ref.get() else {
            return;
        };

        let html_el: &web_sys::HtmlElement = &content_el;
        let Ok(focusable_list) = html_el.query_selector_all(
            "button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"])"
        ) else {
            return;
        };

        let count = focusable_list.length();
        if count == 0 {
            ev.prevent_default();
            return;
        }

        let first = focusable_list.get(0);
        let last = focusable_list.get(count - 1);

        let active = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element());

        if ev.shift_key() {
            // Shift+Tab: if on first element, wrap to last
            if let (Some(first_node), Some(active_el)) = (&first, &active) {
                if let Some(first_el) = first_node.dyn_ref::<web_sys::Element>() {
                    if first_el == active_el {
                        ev.prevent_default();
                        if let Some(last_node) = &last {
                            if let Ok(last_html) =
                                last_node.clone().dyn_into::<web_sys::HtmlElement>()
                            {
                                let _ = last_html.focus();
                            }
                        }
                    }
                }
            }
        } else {
            // Tab: if on last element, wrap to first
            if let (Some(last_node), Some(active_el)) = (&last, &active) {
                if let Some(last_el) = last_node.dyn_ref::<web_sys::Element>() {
                    if last_el == active_el {
                        ev.prevent_default();
                        if let Some(first_node) = &first {
                            if let Ok(first_html) =
                                first_node.clone().dyn_into::<web_sys::HtmlElement>()
                            {
                                let _ = first_html.focus();
                            }
                        }
                    }
                }
            }
        }
    };

    // Prevent clicks inside content from bubbling to overlay
    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };
    let title_id = ctx.title_id.get_value();
    let description_id = ctx.description_id.get_value();

    view! {
        <div
            node_ref=content_ref
            role="dialog"
            aria-modal="true"
            aria-labelledby=title_id
            aria-describedby=description_id
            class=class
            style=style
            tabindex="-1"
            data-radix-dialog-content=""
            data-state=state_attr
            data-dialog-id=ctx.dialog_id.to_string()
            on:keydown=on_keydown
            on:click=on_click
        >
            {children()}
        </div>
    }
}

/// The dialog title.
#[component]
pub fn DialogTitle(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the title element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::H2>,

    /// The title content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogTitle must be used within DialogRoot");

    let title_id = ctx.title_id.get_value();

    view! {
        <h2
            node_ref=node_ref
            id=title_id
            class=class
            style=style
            data-radix-dialog-title=""
        >
            {children()}
        </h2>
    }
}

/// The dialog description.
#[component]
pub fn DialogDescription(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the description element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::P>,

    /// The description content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<DialogContext>().expect("DialogDescription must be used within DialogRoot");

    let description_id = ctx.description_id.get_value();

    view! {
        <p
            node_ref=node_ref
            id=description_id
            class=class
            style=style
            data-radix-dialog-description=""
        >
            {children()}
        </p>
    }
}

/// Close button for the dialog.
/// Uses DOM traversal to find the correct dialog to close, avoiding context issues with nested dialogs.
#[component]
pub fn DialogClose(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The close button content.
    children: ChildrenFn,
) -> impl IntoView {
    let button_ref: NodeRef<Button> = node_ref;

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();

        // Find the closest DialogContent ancestor and get its dialog ID
        if let Some(button) = button_ref.get() {
            let button_el: &web_sys::HtmlElement = &button;
            if let Some(content) = button_el
                .closest("[data-radix-dialog-content]")
                .ok()
                .flatten()
            {
                if let Some(id_str) = content.get_attribute("data-dialog-id") {
                    if let Ok(dialog_id) = id_str.parse::<usize>() {
                        if let Some(signal) = get_dialog_signal(dialog_id) {
                            signal.set(false);
                        }
                    }
                }
            }
        }
    };

    view! {
        <button
            node_ref=button_ref
            type="button"
            class=class
            style=style
            data-radix-dialog-close=""
            on:click=on_click
        >
            {children()}
        </button>
    }
}
