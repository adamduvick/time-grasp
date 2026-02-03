use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::primitives::popper::{
    PopperAlign, PopperAnchor, PopperArrow, PopperContent, PopperContext, PopperRoot, PopperSide,
};

/// Re-export popper types for convenience
pub use crate::primitives::popper::{PopperAlign as PopoverAlign, PopperSide as PopoverSide};

/// Context shared between Popover components.
#[derive(Clone, Copy)]
struct PopoverContext {
    open: RwSignal<bool>,
    /// NodeRef to the trigger element (used for focus return)
    trigger_ref: NodeRef<leptos::html::Button>,
    /// Whether the popover was opened via keyboard
    opened_via_keyboard: RwSignal<bool>,
    /// Whether this popover uses a custom anchor
    has_custom_anchor: RwSignal<bool>,
}

/// Root container for a popover.
#[component]
pub fn PopoverRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// The popover parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));

    // Wrap to notify callback on changes
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    let ctx = PopoverContext {
        open: open_signal,
        trigger_ref: NodeRef::new(),
        opened_via_keyboard: RwSignal::new(false),
        has_custom_anchor: RwSignal::new(false),
    };

    provide_context(ctx);

    view! {
        <PopperRoot>
            {children()}
        </PopperRoot>
    }
}

/// Optional custom anchor element. If not used, PopoverTrigger serves as the anchor.
#[component]
pub fn PopoverAnchor(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The anchor content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<PopoverContext>().expect("PopoverAnchor must be used within PopoverRoot");

    // Mark that we have a custom anchor
    ctx.has_custom_anchor.set(true);

    view! {
        <PopperAnchor class=class.unwrap_or_default() style=style.unwrap_or_default()>
            {children()}
        </PopperAnchor>
    }
}

/// Trigger button that toggles the popover.
#[component]
pub fn PopoverTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether the trigger is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// The trigger content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<PopoverContext>().expect("PopoverTrigger must be used within PopoverRoot");

    let trigger_ref = ctx.trigger_ref;
    let open_signal = ctx.open;
    let opened_via_keyboard = ctx.opened_via_keyboard;
    let has_custom_anchor = ctx.has_custom_anchor;

    let on_click = move |ev: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        // Detect if this was a keyboard-initiated click (detail = 0)
        opened_via_keyboard.set(ev.detail() == 0);
        open_signal.update(|open| *open = !*open);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        // Space and Enter toggle the popover
        if ev.key() == " " || ev.key() == "Enter" {
            ev.prevent_default();
            opened_via_keyboard.set(true);
            open_signal.update(|open| *open = !*open);
        }
    };

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };

    // Render trigger with or without anchor wrapper based on has_custom_anchor
    // We check once at render time since custom anchor is set before trigger
    if has_custom_anchor.get_untracked() {
        view! {
            <button
                type="button"
                node_ref=trigger_ref
                class=class
                style=style
                disabled=move || disabled.get()
                aria-haspopup="dialog"
                aria-expanded=move || open_signal.get().to_string()
                data-radix-popover-trigger=""
                data-state=state_attr
                on:click=on_click
                on:keydown=on_keydown
            >
                {children()}
            </button>
        }.into_any()
    } else {
        view! {
            <PopperAnchor>
                <button
                    type="button"
                    node_ref=trigger_ref
                    class=class
                    style=style
                    disabled=move || disabled.get()
                    aria-haspopup="dialog"
                    aria-expanded=move || open_signal.get().to_string()
                    data-radix-popover-trigger=""
                    data-state=state_attr
                    on:click=on_click
                    on:keydown=on_keydown
                >
                    {children()}
                </button>
            </PopperAnchor>
        }.into_any()
    }
}

/// Portal container - renders popover content to document.body when open.
#[component]
pub fn PopoverPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let popover_ctx =
        use_context::<PopoverContext>().expect("PopoverPortal must be used within PopoverRoot");
    let popper_ctx =
        use_context::<PopperContext>().expect("PopoverPortal must be used within PopoverRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || popover_ctx.open.get()>
            <Portal>
                <PopoverPortalInner popover_ctx=popover_ctx popper_ctx=popper_ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn PopoverPortalInner(
    popover_ctx: PopoverContext,
    popper_ctx: PopperContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(popover_ctx);
    provide_context(popper_ctx);
    children.with_value(|c| c())
}

/// The popover content panel.
#[component]
pub fn PopoverContent(
    /// Which side of the anchor to show the content.
    #[prop(into, default = Signal::derive(|| PopperSide::Bottom))]
    side: Signal<PopperSide>,

    /// Offset from the anchor in pixels.
    #[prop(into, default = Signal::derive(|| 0i32))]
    side_offset: Signal<i32>,

    /// Alignment along the side.
    #[prop(into, default = Signal::derive(|| PopperAlign::Center))]
    align: Signal<PopperAlign>,

    /// Offset along the alignment axis.
    #[prop(into, default = Signal::derive(|| 0i32))]
    align_offset: Signal<i32>,

    /// Whether to avoid collisions with viewport boundaries.
    #[prop(into, default = Signal::derive(|| true))]
    avoid_collisions: Signal<bool>,

    /// Padding from viewport edges for collision detection.
    #[prop(into, default = Signal::derive(|| 0i32))]
    collision_padding: Signal<i32>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles (merged with positioning styles).
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Callback when escape key is pressed.
    #[prop(optional)]
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,

    /// Callback when pointer is pressed outside.
    #[prop(optional)]
    on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,

    /// Callback when focus moves outside.
    #[prop(optional)]
    on_focus_outside: Option<Callback<web_sys::FocusEvent>>,

    /// Callback when interaction occurs outside (pointer or focus).
    #[prop(optional)]
    on_interact_outside: Option<Callback<()>>,

    /// The popover content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<PopoverContext>().expect("PopoverContent must be used within PopoverRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let opened_via_keyboard = ctx.opened_via_keyboard;
    let trigger_ref = ctx.trigger_ref;

    // Focus the content when opened via keyboard
    Effect::new(move || {
        if open_signal.get() && opened_via_keyboard.get() {
            // Use requestAnimationFrame to ensure DOM is ready
            if let Some(window) = web_sys::window() {
                let content_ref_clone = content_ref;
                let callback = Closure::<dyn Fn()>::new(move || {
                    if let Some(el) = content_ref_clone.get() {
                        let html_el: &web_sys::HtmlElement = &el;
                        let _ = html_el.focus();
                    }
                });
                let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                callback.forget();
            }
        }
    });

    // Handle escape key
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" {
            if let Some(callback) = on_escape_key_down {
                callback.run(ev.clone());
            }
            // Only close if not prevented
            if !ev.default_prevented() {
                open_signal.set(false);
                // Return focus to trigger
                if let Some(el) = trigger_ref.get() {
                    let html_el: &web_sys::HtmlElement = &el;
                    let _ = html_el.focus();
                }
            }
        }
    };

    // Handle clicks outside
    Effect::new(move || {
        if !open_signal.get() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let callback = Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |ev: web_sys::PointerEvent| {
            let Some(target) = ev.target() else {
                return;
            };
            let Some(target_node) = target.dyn_ref::<web_sys::Node>() else {
                return;
            };

            // Check if click is inside content
            if let Some(content_el) = content_ref.get() {
                let content_node: &web_sys::Node = &content_el;
                if content_node.contains(Some(target_node)) {
                    return;
                }
            }

            // Check if click is on trigger
            if let Some(trigger_el) = trigger_ref.get() {
                let trigger_node: &web_sys::Node = &trigger_el;
                if trigger_node.contains(Some(target_node)) {
                    return;
                }
            }

            // Click was outside
            if let Some(callback) = on_pointer_down_outside {
                callback.run(ev);
            }
            if let Some(callback) = on_interact_outside {
                callback.run(());
            }
            open_signal.set(false);
        });

        let _ = document.add_event_listener_with_callback(
            "pointerdown",
            callback.as_ref().unchecked_ref(),
        );

        callback.forget();
    });

    // Handle focus outside
    Effect::new(move || {
        if !open_signal.get() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let callback = Closure::<dyn Fn(web_sys::FocusEvent)>::new(move |ev: web_sys::FocusEvent| {
            let Some(related_target) = ev.related_target() else {
                return;
            };
            let Some(target_node) = related_target.dyn_ref::<web_sys::Node>() else {
                return;
            };

            // Check if focus moved inside content
            if let Some(content_el) = content_ref.get() {
                let content_node: &web_sys::Node = &content_el;
                if content_node.contains(Some(target_node)) {
                    return;
                }
            }

            // Check if focus moved to trigger
            if let Some(trigger_el) = trigger_ref.get() {
                let trigger_node: &web_sys::Node = &trigger_el;
                if trigger_node.contains(Some(target_node)) {
                    return;
                }
            }

            // Focus moved outside
            if let Some(callback) = on_focus_outside {
                callback.run(ev);
            }
            if let Some(callback) = on_interact_outside {
                callback.run(());
            }
        });

        let _ = document.add_event_listener_with_callback_and_bool(
            "focusout",
            callback.as_ref().unchecked_ref(),
            true, // capture phase
        );

        callback.forget();
    });

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };

    // Store class for use in closure
    let class_val = StoredValue::new(class.unwrap_or_default());

    view! {
        <PopperContent
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_padding=collision_padding
            style=style.unwrap_or_default()
        >
            <div
                node_ref=content_ref
                role="dialog"
                tabindex="-1"
                class=class_val.get_value()
                data-radix-popover-content=""
                data-state=state_attr
                on:keydown=handle_keydown
            >
                {children.with_value(|c| c())}
            </div>
        </PopperContent>
    }
}

/// Arrow component that points to the anchor.
#[component]
pub fn PopoverArrow(
    /// Width of the arrow in pixels.
    #[prop(default = 10)]
    width: u32,

    /// Height of the arrow in pixels.
    #[prop(default = 5)]
    height: u32,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <PopperArrow width=width height=height class=class.unwrap_or_default() />
    }
}

/// Close button for the popover.
#[component]
pub fn PopoverClose(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the close button.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Button>,

    /// The close button content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<PopoverContext>().expect("PopoverClose must be used within PopoverRoot");

    let trigger_ref = ctx.trigger_ref;
    let open_signal = ctx.open;

    let on_click = move |_: web_sys::MouseEvent| {
        open_signal.set(false);
        // Return focus to trigger
        if let Some(el) = trigger_ref.get() {
            let html_el: &web_sys::HtmlElement = &el;
            let _ = html_el.focus();
        }
    };

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            style=style
            data-radix-popover-close=""
            on:click=on_click
        >
            {children()}
        </button>
    }
}
