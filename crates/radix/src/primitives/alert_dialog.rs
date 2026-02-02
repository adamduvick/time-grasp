use leptos::html::Button;
use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

// TODO, does it make sense to share code with the Dialog codebase or add some utilities?
// it could make more sense to leave the two decoupled to leave the option open
// to include them atomically in projects

static ALERT_DIALOG_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Global registry of alert dialog open signals, keyed by dialog ID
thread_local! {
    static ALERT_DIALOG_REGISTRY: std::cell::RefCell<std::collections::HashMap<usize, RwSignal<bool>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn register_alert_dialog(id: usize, open: RwSignal<bool>) {
    ALERT_DIALOG_REGISTRY.with(|r| {
        r.borrow_mut().insert(id, open);
    });
}

fn unregister_alert_dialog(id: usize) {
    ALERT_DIALOG_REGISTRY.with(|r| {
        r.borrow_mut().remove(&id);
    });
}

fn get_alert_dialog_signal(id: usize) -> Option<RwSignal<bool>> {
    ALERT_DIALOG_REGISTRY.with(|r| r.borrow().get(&id).copied())
}

/// Context shared between AlertDialog components.
#[derive(Clone, Copy)]
struct AlertDialogContext {
    open: RwSignal<bool>,
    title_id: StoredValue<String>,
    description_id: StoredValue<String>,
    dialog_id: usize,
}

/// Root container for an alert dialog.
/// Unlike Dialog, AlertDialog requires explicit action to close (no click-outside dismiss).
#[component]
pub fn AlertDialogRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Callback when open state changes.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,

    /// The alert dialog parts.
    children: ChildrenFn,
) -> impl IntoView {
    let id = ALERT_DIALOG_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let title_id = StoredValue::new(format!("alert-dialog-title-{}", id));
    let description_id = StoredValue::new(format!("alert-dialog-description-{}", id));

    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));

    // Register this dialog's signal in the global registry
    register_alert_dialog(id, open_signal);

    // Unregister when component is cleaned up
    on_cleanup(move || {
        unregister_alert_dialog(id);
    });

    // Notify on_open_change when open state changes
    if let Some(callback) = on_open_change {
        Effect::new(move |_| {
            callback.run(open_signal.get());
        });
    }

    let ctx = AlertDialogContext {
        open: open_signal,
        title_id,
        description_id,
        dialog_id: id,
    };

    provide_context(ctx);

    children()
}

/// Trigger button that opens the alert dialog.
#[component]
pub fn AlertDialogTrigger(
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
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogTrigger must be used within AlertDialogRoot");

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
            data-radix-alert-dialog-trigger=""
            data-state=state_attr
            data-alert-dialog-id=ctx.dialog_id.to_string()
            aria-haspopup="dialog"
            aria-expanded=move || ctx.open.get().to_string()
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Portal container - renders children to document.body when alert dialog is open.
#[component]
pub fn AlertDialogPortal(
    /// The portal content (overlay and content).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogPortal must be used within AlertDialogRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <Portal>
                <AlertDialogPortalInner ctx=ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn AlertDialogPortalInner(
    ctx: AlertDialogContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    // Re-provide context inside the portal
    provide_context(ctx);

    children.with_value(|c| c())
}

/// Overlay backdrop behind the alert dialog.
/// Unlike Dialog, clicking the overlay does NOT close the alert dialog.
#[component]
pub fn AlertDialogOverlay(
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
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogOverlay must be used within AlertDialogRoot");

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };

    // No on_click handler - clicking overlay does not close alert dialog
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-alert-dialog-overlay=""
            data-state=state_attr
            data-alert-dialog-id=ctx.dialog_id.to_string()
        />
    }
}

/// The alert dialog content container.
#[component]
pub fn AlertDialogContent(
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
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogContent must be used within AlertDialogRoot");

    let content_ref = node_ref;
    let dialog_id = ctx.dialog_id;

    // Store the trigger selector for focus restoration when dialog closes
    let trigger_selector = format!(
        "[data-radix-alert-dialog-trigger][data-alert-dialog-id=\"{}\"]",
        dialog_id
    );

    // Focus management - focus the Cancel button when mounted, or first focusable element
    Effect::new(move |_| {
        if let Some(el) = content_ref.get() {
            let html_el: &web_sys::HtmlElement = &el;

            // Try to focus the Cancel button first (preferred for AlertDialog)
            if let Ok(Some(cancel)) = html_el.query_selector("[data-radix-alert-dialog-cancel]") {
                if let Ok(cancel_el) = cancel.dyn_into::<web_sys::HtmlElement>() {
                    let _ = cancel_el.focus();
                    return;
                }
            }

            // Fallback to first focusable element
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
                    "[data-radix-alert-dialog-content][data-alert-dialog-id=\"{}\"]",
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
                    // Focus is on body - check if this is the topmost open alert dialog
                    let all_dialogs = web_sys::window().and_then(|w| w.document()).and_then(|d| {
                        d.query_selector_all("[data-radix-alert-dialog-content]")
                            .ok()
                    });

                    match all_dialogs {
                        Some(list) if list.length() > 0 => list
                            .get(list.length() - 1)
                            .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
                            .map(|el| el == content)
                            .unwrap_or(false),
                        _ => true,
                    }
                } else {
                    false
                };

                if !should_handle {
                    return;
                }

                // If focus is on an input-type element, just blur it
                if let Some(ref active) = active_element {
                    let tag_name = active.tag_name().to_uppercase();
                    if tag_name == "INPUT" || tag_name == "TEXTAREA" || tag_name == "SELECT" {
                        ev.prevent_default();
                        ev.stop_propagation();
                        if let Ok(html_active) = active.clone().dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_active.blur();
                        }
                        return;
                    }
                }

                // Close the dialog
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(signal) = get_alert_dialog_signal(dialog_id) {
                    signal.set(false);
                }
            });

        let _ = doc.add_event_listener_with_callback_and_bool(
            "keydown",
            handler.as_ref().unchecked_ref(),
            true, // capture phase
        );

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

    // Prevent clicks inside content from bubbling
    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
    };

    let state_attr = move || if ctx.open.get() { "open" } else { "closed" };
    let title_id = ctx.title_id.get_value();
    let description_id = ctx.description_id.get_value();

    view! {
        <div
            node_ref=content_ref
            role="alertdialog"
            aria-modal="true"
            aria-labelledby=title_id
            aria-describedby=description_id
            class=class
            style=style
            tabindex="-1"
            data-radix-alert-dialog-content=""
            data-state=state_attr
            data-alert-dialog-id=ctx.dialog_id.to_string()
            on:keydown=on_keydown
            on:click=on_click
        >
            {children()}
        </div>
    }
}

/// The alert dialog title.
#[component]
pub fn AlertDialogTitle(
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
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogTitle must be used within AlertDialogRoot");

    let title_id = ctx.title_id.get_value();

    view! {
        <h2
            node_ref=node_ref
            id=title_id
            class=class
            style=style
            data-radix-alert-dialog-title=""
        >
            {children()}
        </h2>
    }
}

/// The alert dialog description.
#[component]
pub fn AlertDialogDescription(
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
    let ctx = use_context::<AlertDialogContext>()
        .expect("AlertDialogDescription must be used within AlertDialogRoot");

    let description_id = ctx.description_id.get_value();

    view! {
        <p
            node_ref=node_ref
            id=description_id
            class=class
            style=style
            data-radix-alert-dialog-description=""
        >
            {children()}
        </p>
    }
}

/// Action button that closes the alert dialog.
/// Use this for the primary/destructive action (e.g., "Delete", "Confirm").
#[component]
pub fn AlertDialogAction(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The action button content.
    children: ChildrenFn,
) -> impl IntoView {
    let button_ref: NodeRef<Button> = node_ref;

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();

        // Find the closest AlertDialogContent ancestor and get its dialog ID
        if let Some(button) = button_ref.get() {
            let button_el: &web_sys::HtmlElement = &button;
            if let Some(content) = button_el
                .closest("[data-radix-alert-dialog-content]")
                .ok()
                .flatten()
            {
                if let Some(id_str) = content.get_attribute("data-alert-dialog-id") {
                    if let Ok(dialog_id) = id_str.parse::<usize>() {
                        if let Some(signal) = get_alert_dialog_signal(dialog_id) {
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
            data-radix-alert-dialog-action=""
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Cancel button that closes the alert dialog.
/// This button receives initial focus when the alert dialog opens.
#[component]
pub fn AlertDialogCancel(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The cancel button content.
    children: ChildrenFn,
) -> impl IntoView {
    let button_ref: NodeRef<Button> = node_ref;

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();

        // Find the closest AlertDialogContent ancestor and get its dialog ID
        if let Some(button) = button_ref.get() {
            let button_el: &web_sys::HtmlElement = &button;
            if let Some(content) = button_el
                .closest("[data-radix-alert-dialog-content]")
                .ok()
                .flatten()
            {
                if let Some(id_str) = content.get_attribute("data-alert-dialog-id") {
                    if let Ok(dialog_id) = id_str.parse::<usize>() {
                        if let Some(signal) = get_alert_dialog_signal(dialog_id) {
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
            data-radix-alert-dialog-cancel=""
            on:click=on_click
        >
            {children()}
        </button>
    }
}
