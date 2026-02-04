use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// Counter for generating unique viewport IDs
static VIEWPORT_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

// Counter for generating unique toast IDs
static TOAST_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Swipe direction for dismissing toasts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastSwipeDirection {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl ToastSwipeDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastSwipeDirection::Right => "right",
            ToastSwipeDirection::Left => "left",
            ToastSwipeDirection::Up => "up",
            ToastSwipeDirection::Down => "down",
        }
    }
}

/// Entry for a registered toast in the stack
#[derive(Clone, Copy)]
struct ToastEntry {
    id: u32,
    open: RwSignal<bool>,
    on_open_change: Option<Callback<bool>>,
}

/// Context shared between Toast components.
#[derive(Clone, Copy)]
struct ToastProviderContext {
    swipe_direction: Signal<ToastSwipeDirection>,
    swipe_threshold: Signal<i32>,
    default_duration: Signal<u32>,
    /// The viewport element where toasts will be portaled into
    viewport_id: StoredValue<String>,
    /// Stack of open toasts (most recent last) for Escape key handling
    toast_stack: RwSignal<Vec<ToastEntry>>,
}

/// Context for individual toast.
#[derive(Clone, Copy)]
struct ToastContext {
    open: RwSignal<bool>,
    on_open_change: Option<Callback<bool>>,
}

/// Provider for Toast components. Wraps the application area where toasts can appear.
#[component]
pub fn ToastProvider(
    /// Direction to swipe for dismissing toasts.
    #[prop(into, default = Signal::derive(|| ToastSwipeDirection::Right))]
    swipe_direction: Signal<ToastSwipeDirection>,

    /// Distance in pixels required to dismiss via swipe.
    #[prop(into, default = Signal::derive(|| 50i32))]
    swipe_threshold: Signal<i32>,

    /// Default duration in ms before auto-dismiss.
    #[prop(into, default = Signal::derive(|| 5000u32))]
    duration: Signal<u32>,

    /// The content.
    children: Children,
) -> impl IntoView {
    // Generate a unique ID for the viewport element
    let id = VIEWPORT_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let viewport_id = StoredValue::new(format!("radix-toast-viewport-{}", id));

    // Stack of open toasts for Escape key handling
    let toast_stack: RwSignal<Vec<ToastEntry>> = RwSignal::new(Vec::new());

    let ctx = ToastProviderContext {
        swipe_direction,
        swipe_threshold,
        default_duration: duration,
        viewport_id,
        toast_stack,
    };

    provide_context(ctx);

    // Set up document-level Escape key listener
    Effect::new(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let callback = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                // Get the most recent toast from the stack
                let stack = toast_stack.get();
                if let Some(entry) = stack.last() {
                    // Check if the toast is still open
                    if let Some(is_open) = entry.open.try_get() {
                        if is_open {
                            ev.prevent_default();
                            let _ = entry.open.try_set(false);
                            if let Some(cb) = entry.on_open_change {
                                cb.run(false);
                            }
                        }
                    }
                }
            }
        });

        let _ = document.add_event_listener_with_callback(
            "keydown",
            callback.as_ref().unchecked_ref(),
        );

        callback.forget();
    });

    children()
}

/// Individual toast notification.
#[component]
pub fn ToastRoot(
    /// Controlled open state.
    #[prop(optional, into)]
    open: Option<RwSignal<bool>>,

    /// Default open state (uncontrolled).
    #[prop(default = true)]
    default_open: bool,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// Duration in ms before auto-dismiss. Use 0 or very large number to disable.
    #[prop(optional, into)]
    duration: Option<Signal<u32>>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The toast content.
    children: ChildrenFn,
) -> impl IntoView {
    let provider_ctx = use_context::<ToastProviderContext>()
        .expect("ToastRoot must be used within ToastProvider");

    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));
    let duration_signal = duration.unwrap_or(provider_ctx.default_duration);

    // Generate unique ID for this toast
    let toast_id = TOAST_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    // Timer ID stored in signal (use i32, -1 means no timer)
    let timer_id = RwSignal::new(-1i32);
    let is_paused = RwSignal::new(false);

    // Register/unregister with toast stack based on open state
    let toast_stack = provider_ctx.toast_stack;
    Effect::new(move || {
        let is_open = open_signal.get();
        if is_open {
            // Add to stack when opened
            toast_stack.update(|stack| {
                // Remove any existing entry for this toast (in case of re-open)
                stack.retain(|e| e.id != toast_id);
                stack.push(ToastEntry {
                    id: toast_id,
                    open: open_signal,
                    on_open_change,
                });
            });
        } else {
            // Remove from stack when closed
            toast_stack.update(|stack| {
                stack.retain(|e| e.id != toast_id);
            });
        }
    });

    // Helper to clear existing timer
    let clear_timer = move || {
        let id = timer_id.get_untracked();
        if id >= 0 {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            timer_id.set(-1);
        }
    };

    // Start the auto-dismiss timer
    let start_timer = move || {
        clear_timer();

        let duration_ms = duration_signal.get_untracked();
        if duration_ms == 0 {
            return;
        }

        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if let Some(is_open) = open_signal.try_get_untracked() {
                    if is_open {
                        let _ = open_signal.try_set(false);
                        if let Some(cb) = on_open_change {
                            cb.run(false);
                        }
                    }
                }
            });

            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                duration_ms as i32,
            ) {
                timer_id.set(id);
            }

            callback.forget();
        }
    };

    // Start timer when toast opens
    Effect::new(move || {
        if open_signal.get() && !is_paused.get() {
            start_timer();
        }
    });

    let toast_ctx = ToastContext {
        open: open_signal,
        on_open_change,
    };

    provide_context(toast_ctx);

    // Swipe handling state
    let swipe_start = RwSignal::new(None::<(i32, i32)>);
    let swipe_delta = RwSignal::new((0i32, 0i32));
    let swipe_state = RwSignal::new("idle"); // idle, start, move, cancel, end

    let swipe_direction = provider_ctx.swipe_direction;
    let swipe_threshold = provider_ctx.swipe_threshold;

    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        swipe_start.set(Some((ev.client_x(), ev.client_y())));
        swipe_state.set("start");
        clear_timer();
        is_paused.set(true);
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        let Some((start_x, start_y)) = swipe_start.get() else {
            return;
        };

        let dx = ev.client_x() - start_x;
        let dy = ev.client_y() - start_y;

        // Only track movement in the swipe direction
        let delta = match swipe_direction.get() {
            ToastSwipeDirection::Right => (dx.max(0), 0),
            ToastSwipeDirection::Left => (dx.min(0), 0),
            ToastSwipeDirection::Down => (0, dy.max(0)),
            ToastSwipeDirection::Up => (0, dy.min(0)),
        };

        swipe_delta.set(delta);
        if delta.0 != 0 || delta.1 != 0 {
            swipe_state.set("move");
        }
    };

    let on_pointer_up = move |_: web_sys::PointerEvent| {
        let (dx, dy) = swipe_delta.get();
        let threshold = swipe_threshold.get();

        let should_dismiss = match swipe_direction.get() {
            ToastSwipeDirection::Right => dx >= threshold,
            ToastSwipeDirection::Left => dx <= -threshold,
            ToastSwipeDirection::Down => dy >= threshold,
            ToastSwipeDirection::Up => dy <= -threshold,
        };

        if should_dismiss {
            swipe_state.set("end");
            let _ = open_signal.try_set(false);
            if let Some(cb) = on_open_change {
                cb.run(false);
            }
        } else {
            swipe_state.set("cancel");
            is_paused.set(false);
            start_timer();
        }

        swipe_start.set(None);
        swipe_delta.set((0, 0));
    };

    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        clear_timer();
        is_paused.set(true);
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        if swipe_start.get().is_none() {
            is_paused.set(false);
            start_timer();
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        clear_timer();
        is_paused.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        if swipe_start.get().is_none() {
            is_paused.set(false);
            start_timer();
        }
    };

    let data_state = move || if open_signal.get() { "open" } else { "closed" };
    let data_swipe = move || swipe_state.get();

    let class_val = StoredValue::new(class.unwrap_or_default());
    let style_val = StoredValue::new(style.unwrap_or_default());

    let combined_style = move || {
        let (dx, dy) = swipe_delta.get();
        let user_style = style_val.get_value();
        let swipe_style = format!(
            "--radix-toast-swipe-move-x: {}px; --radix-toast-swipe-move-y: {}px;",
            dx, dy
        );
        if user_style.is_empty() {
            swipe_style
        } else {
            format!("{} {}", swipe_style, user_style)
        }
    };

    let children = StoredValue::new(children);
    let viewport_id = provider_ctx.viewport_id;

    // Get the viewport element to portal into
    let get_viewport = move || {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&viewport_id.get_value()))
    };

    view! {
        <Show when=move || open_signal.get() && get_viewport().is_some()>
            <Portal mount=get_viewport().unwrap()>
                <ToastRootInner
                    toast_ctx=toast_ctx
                    class=class_val.get_value()
                    combined_style=combined_style
                    data_state=data_state
                    data_swipe=data_swipe
                    on_pointer_down=on_pointer_down
                    on_pointer_move=on_pointer_move
                    on_pointer_up=on_pointer_up
                    on_mouse_enter=on_mouse_enter
                    on_mouse_leave=on_mouse_leave
                    on_focus=on_focus
                    on_blur=on_blur
                    children=children
                />
            </Portal>
        </Show>
    }
}

/// Inner component that provides context inside the Portal scope
#[component]
fn ToastRootInner(
    toast_ctx: ToastContext,
    class: String,
    #[prop(into)] combined_style: Signal<String>,
    #[prop(into)] data_state: Signal<&'static str>,
    #[prop(into)] data_swipe: Signal<&'static str>,
    on_pointer_down: impl Fn(web_sys::PointerEvent) + 'static,
    on_pointer_move: impl Fn(web_sys::PointerEvent) + 'static,
    on_pointer_up: impl Fn(web_sys::PointerEvent) + 'static,
    on_mouse_enter: impl Fn(web_sys::MouseEvent) + 'static,
    on_mouse_leave: impl Fn(web_sys::MouseEvent) + 'static,
    on_focus: impl Fn(web_sys::FocusEvent) + 'static,
    on_blur: impl Fn(web_sys::FocusEvent) + 'static,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    // Provide context inside the Portal's reactive scope
    provide_context(toast_ctx);

    view! {
        <li
            role="status"
            aria-live="polite"
            aria-atomic="true"
            tabindex="0"
            class=class
            style=move || combined_style.get()
            data-radix-toast-root=""
            data-state=move || data_state.get()
            data-swipe=move || data_swipe.get()
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            on:focus=on_focus
            on:blur=on_blur
        >
            {children.with_value(|c| c())}
        </li>
    }
}

/// Title of the toast.
#[component]
pub fn ToastTitle(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The title content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-toast-title=""
        >
            {children()}
        </div>
    }
}

/// Description text of the toast.
#[component]
pub fn ToastDescription(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The description content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-toast-description=""
        >
            {children()}
        </div>
    }
}

/// Action button in the toast.
#[component]
pub fn ToastAction(
    /// Alternative text for accessibility (required).
    #[prop(into)]
    alt_text: String,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The action content.
    children: Children,
) -> impl IntoView {
    let alt_text = StoredValue::new(alt_text);

    view! {
        <button
            type="button"
            class=class
            style=style
            data-radix-toast-action=""
            aria-label=alt_text.get_value()
        >
            {children()}
        </button>
    }
}

/// Close button for the toast.
#[component]
pub fn ToastClose(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The close button content.
    children: Children,
) -> impl IntoView {
    let toast_ctx =
        use_context::<ToastContext>().expect("ToastClose must be used within ToastRoot");

    let on_click = move |ev: web_sys::MouseEvent| {
        // Stop propagation to prevent swipe handler interference
        ev.stop_propagation();
        toast_ctx.open.set(false);
        if let Some(cb) = toast_ctx.on_open_change {
            cb.run(false);
        }
    };

    // Stop pointer events from triggering swipe detection on parent
    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        ev.stop_propagation();
    };

    view! {
        <button
            type="button"
            class=class
            style=style
            data-radix-toast-close=""
            aria-label="Close"
            on:click=on_click
            on:pointerdown=on_pointer_down
        >
            {children()}
        </button>
    }
}

/// Viewport where toasts are rendered. Should be placed at the end of the Provider.
#[component]
pub fn ToastViewport(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Label for screen readers.
    #[prop(into, default = "Notifications".to_string())]
    label: String,

    /// Hotkey to focus the viewport (e.g., "F8").
    #[prop(optional, into)]
    hotkey: Option<String>,
) -> impl IntoView {
    let provider_ctx = use_context::<ToastProviderContext>()
        .expect("ToastViewport must be used within ToastProvider");

    let viewport_ref: NodeRef<leptos::html::Ol> = NodeRef::new();
    let label = StoredValue::new(label);
    let class = StoredValue::new(class);
    let style = StoredValue::new(style);
    let viewport_id = provider_ctx.viewport_id;

    // Set up hotkey listener if provided
    if let Some(key) = hotkey {
        let key = StoredValue::new(key);
        Effect::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };

            let viewport_ref_inner = viewport_ref;
            let callback = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() == key.get_value() {
                    if let Some(Some(el)) = viewport_ref_inner.try_get_untracked() {
                        let html_el: &web_sys::HtmlElement = &el;
                        let _ = html_el.focus();
                    }
                }
            });

            let _ = window.add_event_listener_with_callback(
                "keydown",
                callback.as_ref().unchecked_ref(),
            );

            callback.forget();
        });
    }

    view! {
        <Portal>
            <ol
                node_ref=viewport_ref
                id=viewport_id.get_value()
                role="region"
                aria-label=label.get_value()
                tabindex="-1"
                class=class.get_value()
                style=style.get_value()
                data-radix-toast-viewport=""
            />
        </Portal>
    }
}
