use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::primitives::popper::{
    PopperAlign, PopperAnchor, PopperArrow, PopperContent, PopperContext, PopperRoot, PopperSide,
};

/// Re-export popper types for convenience
pub use crate::primitives::popper::{PopperAlign as TooltipAlign, PopperSide as TooltipSide};

// Global timestamp of when any tooltip was last open (for skip delay behavior)
static LAST_TOOLTIP_CLOSE_TIME: AtomicU64 = AtomicU64::new(0);

fn current_time_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// Provider context for tooltip delay settings.
#[derive(Clone, Copy)]
struct TooltipProviderContext {
    delay_duration: u32,
    skip_delay_duration: u32,
}

/// Provider that wraps tooltips to share delay settings.
#[component]
pub fn TooltipProvider(
    /// Delay in milliseconds before tooltip shows on hover.
    #[prop(default = 700)]
    delay_duration: u32,

    /// Duration in milliseconds to skip delay when moving between tooltips.
    #[prop(default = 300)]
    skip_delay_duration: u32,

    /// Children that can use tooltips.
    children: Children,
) -> impl IntoView {
    provide_context(TooltipProviderContext {
        delay_duration,
        skip_delay_duration,
    });

    children()
}

/// Context shared between Tooltip components.
#[derive(Clone, Copy)]
struct TooltipContext {
    open: RwSignal<bool>,
    delay_duration: u32,
    skip_delay_duration: u32,
    /// Tracks whether the tooltip opened with a delay or instantly
    was_instant: RwSignal<bool>,
    /// Whether pointer is currently over trigger or content (grace period for moving between them)
    is_pointer_inside: RwSignal<bool>,
}

/// Root container for a tooltip.
#[component]
pub fn TooltipRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Override delay duration for this specific tooltip.
    #[prop(optional)]
    delay_duration: Option<u32>,

    /// The tooltip parts.
    children: Children,
) -> impl IntoView {
    let provider_ctx = use_context::<TooltipProviderContext>();

    let delay = delay_duration
        .or(provider_ctx.map(|p| p.delay_duration))
        .unwrap_or(700);

    let skip_delay = provider_ctx.map(|p| p.skip_delay_duration).unwrap_or(300);

    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));
    let was_instant = RwSignal::new(false);
    let is_pointer_inside = RwSignal::new(false);

    let ctx = TooltipContext {
        open: open_signal,
        delay_duration: delay,
        skip_delay_duration: skip_delay,
        was_instant,
        is_pointer_inside,
    };

    provide_context(ctx);

    view! {
        <PopperRoot>
            {children()}
        </PopperRoot>
    }
}

/// Trigger element that shows tooltip on hover/focus.
#[component]
pub fn TooltipTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The trigger content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipTrigger must be used within TooltipRoot");

    let hover_timeout: StoredValue<Option<i32>> = StoredValue::new(None);
    let close_timeout: StoredValue<Option<i32>> = StoredValue::new(None);

    let open_signal = ctx.open;
    let was_instant = ctx.was_instant;
    let is_pointer_inside = ctx.is_pointer_inside;
    let delay_duration = ctx.delay_duration;
    let skip_delay_duration = ctx.skip_delay_duration;

    let clear_timeout = move || {
        if let Some(id) = hover_timeout.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            hover_timeout.set_value(None);
        }
    };

    let clear_close_timeout = move || {
        if let Some(id) = close_timeout.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            close_timeout.set_value(None);
        }
    };

    let open_tooltip = move |instant: bool| {
        was_instant.set(instant);
        open_signal.set(true);
    };

    let schedule_close = move || {
        clear_close_timeout();
        // Small grace period to allow moving to content
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                // Only close if pointer is not inside trigger or content
                if !is_pointer_inside.get() {
                    if open_signal.get() {
                        LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
                    }
                    open_signal.set(false);
                }
            });

            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100, // 100ms grace period
            ) {
                close_timeout.set_value(Some(id));
            }

            callback.forget();
        }
    };

    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        clear_timeout();
        clear_close_timeout();
        is_pointer_inside.set(true);

        // Check if we should skip the delay (recently viewed another tooltip)
        let last_close = LAST_TOOLTIP_CLOSE_TIME.load(Ordering::Relaxed);
        let now = current_time_ms();
        let skip_delay = now.saturating_sub(last_close) < skip_delay_duration as u64;

        if delay_duration == 0 || skip_delay {
            open_tooltip(true);
        } else {
            // Set up delayed open
            if let Some(window) = web_sys::window() {
                let callback = Closure::<dyn Fn()>::new(move || {
                    open_tooltip(false);
                });

                if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    delay_duration as i32,
                ) {
                    hover_timeout.set_value(Some(id));
                }

                callback.forget();
            }
        }
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        clear_timeout();
        is_pointer_inside.set(false);
        // Update timestamp immediately when leaving an open tooltip (for skip delay behavior)
        if open_signal.get() {
            LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
        }
        schedule_close();
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        // Focus always opens immediately (keyboard users shouldn't wait)
        open_tooltip(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        clear_timeout();
        if open_signal.get() {
            LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
        }
        open_signal.set(false);
    };

    // Handle Escape key to close tooltip
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" && open_signal.get() {
            ev.prevent_default();
            LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
            open_signal.set(false);
        }
    };

    let state_attr = move || {
        if open_signal.get() {
            if was_instant.get() {
                "instant-open"
            } else {
                "delayed-open"
            }
        } else {
            "closed"
        }
    };

    view! {
        <PopperAnchor class=class.unwrap_or_default() style=style.unwrap_or_default()>
            <span
                data-radix-tooltip-trigger=""
                data-state=state_attr
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
                on:focus=on_focus
                on:blur=on_blur
                on:keydown=on_keydown
            >
                {children()}
            </span>
        </PopperAnchor>
    }
}

/// Portal container - renders tooltip to document.body when open.
#[component]
pub fn TooltipPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let tooltip_ctx =
        use_context::<TooltipContext>().expect("TooltipPortal must be used within TooltipRoot");
    let popper_ctx =
        use_context::<PopperContext>().expect("TooltipPortal must be used within TooltipRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || tooltip_ctx.open.get()>
            <Portal>
                <TooltipPortalInner tooltip_ctx=tooltip_ctx popper_ctx=popper_ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn TooltipPortalInner(
    tooltip_ctx: TooltipContext,
    popper_ctx: PopperContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(tooltip_ctx);
    provide_context(popper_ctx);
    children.with_value(|c| c())
}

/// The tooltip content.
#[component]
pub fn TooltipContent(
    /// Which side of the trigger to show the tooltip.
    #[prop(into, default = Signal::derive(|| PopperSide::Top))]
    side: Signal<PopperSide>,

    /// Offset from the trigger in pixels.
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

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The tooltip content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipContent must be used within TooltipRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let was_instant = ctx.was_instant;
    let is_pointer_inside = ctx.is_pointer_inside;

    // Mouse handlers to keep tooltip open when hovering over content
    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(true);
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(false);
        // Schedule close with grace period
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if !is_pointer_inside.get() {
                    if open_signal.get() {
                        LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
                    }
                    open_signal.set(false);
                }
            });

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100, // 100ms grace period
            );

            callback.forget();
        }
    };

    let state_attr = move || {
        if open_signal.get() {
            if was_instant.get() {
                "instant-open"
            } else {
                "delayed-open"
            }
        } else {
            "closed"
        }
    };

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
                role="tooltip"
                class=class_val.get_value()
                data-radix-tooltip-content=""
                data-state=state_attr
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {children.with_value(|c| c())}
            </div>
        </PopperContent>
    }
}

/// Arrow component that points to the trigger.
#[component]
pub fn TooltipArrow(
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
