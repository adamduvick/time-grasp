use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::primitives::popper::{
    PopperAlign, PopperAnchor, PopperArrow, PopperContent, PopperContext, PopperRoot, PopperSide,
};

/// Re-export popper types for convenience
pub use crate::primitives::popper::{PopperAlign as HoverCardAlign, PopperSide as HoverCardSide};

/// Context shared between HoverCard components.
#[derive(Clone, Copy)]
struct HoverCardContext {
    open: RwSignal<bool>,
    open_delay: u32,
    close_delay: u32,
    /// Whether pointer is currently over trigger or content
    is_pointer_inside: RwSignal<bool>,
}

/// Root container for a hover card.
#[component]
pub fn HoverCardRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Delay in milliseconds before hover card opens.
    #[prop(default = 700)]
    open_delay: u32,

    /// Delay in milliseconds before hover card closes.
    #[prop(default = 300)]
    close_delay: u32,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// The hover card parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));
    let is_pointer_inside = RwSignal::new(false);

    // Notify callback on changes
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    let ctx = HoverCardContext {
        open: open_signal,
        open_delay,
        close_delay,
        is_pointer_inside,
    };

    provide_context(ctx);

    view! {
        <PopperRoot>
            {children()}
        </PopperRoot>
    }
}

/// Trigger element that shows hover card on hover.
#[component]
pub fn HoverCardTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The trigger content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<HoverCardContext>()
        .expect("HoverCardTrigger must be used within HoverCardRoot");

    let open_timeout: StoredValue<Option<i32>> = StoredValue::new(None);
    let close_timeout: StoredValue<Option<i32>> = StoredValue::new(None);

    let open_signal = ctx.open;
    let is_pointer_inside = ctx.is_pointer_inside;
    let open_delay = ctx.open_delay;
    let close_delay = ctx.close_delay;

    let clear_open_timeout = move || {
        if let Some(id) = open_timeout.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            open_timeout.set_value(None);
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

    let schedule_open = move || {
        clear_close_timeout();
        clear_open_timeout();

        if open_delay == 0 {
            open_signal.set(true);
        } else if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if is_pointer_inside.get() {
                    open_signal.set(true);
                }
            });

            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                open_delay as i32,
            ) {
                open_timeout.set_value(Some(id));
            }

            callback.forget();
        }
    };

    let schedule_close = move || {
        clear_open_timeout();
        clear_close_timeout();

        if close_delay == 0 {
            if !is_pointer_inside.get() {
                open_signal.set(false);
            }
        } else if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if !is_pointer_inside.get() {
                    open_signal.set(false);
                }
            });

            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                close_delay as i32,
            ) {
                close_timeout.set_value(Some(id));
            }

            callback.forget();
        }
    };

    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(true);
        schedule_open();
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(false);
        schedule_close();
    };

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };

    view! {
        <PopperAnchor class=class.unwrap_or_default() style=style.unwrap_or_default()>
            <span
                data-radix-hover-card-trigger=""
                data-state=state_attr
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {children()}
            </span>
        </PopperAnchor>
    }
}

/// Portal container - renders hover card to document.body when open.
#[component]
pub fn HoverCardPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let hover_card_ctx = use_context::<HoverCardContext>()
        .expect("HoverCardPortal must be used within HoverCardRoot");
    let popper_ctx =
        use_context::<PopperContext>().expect("HoverCardPortal must be used within HoverCardRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || hover_card_ctx.open.get()>
            <Portal>
                <HoverCardPortalInner hover_card_ctx=hover_card_ctx popper_ctx=popper_ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn HoverCardPortalInner(
    hover_card_ctx: HoverCardContext,
    popper_ctx: PopperContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(hover_card_ctx);
    provide_context(popper_ctx);
    children.with_value(|c| c())
}

/// The hover card content.
#[component]
pub fn HoverCardContent(
    /// Which side of the trigger to show the hover card.
    #[prop(into, default = Signal::derive(|| PopperSide::Bottom))]
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

    /// The hover card content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<HoverCardContext>()
        .expect("HoverCardContent must be used within HoverCardRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let is_pointer_inside = ctx.is_pointer_inside;
    let close_delay = ctx.close_delay;

    let close_timeout: StoredValue<Option<i32>> = StoredValue::new(None);

    let clear_close_timeout = move || {
        if let Some(id) = close_timeout.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            close_timeout.set_value(None);
        }
    };

    let schedule_close = move || {
        clear_close_timeout();

        if close_delay == 0 {
            if !is_pointer_inside.get() {
                open_signal.set(false);
            }
        } else if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if !is_pointer_inside.get() {
                    open_signal.set(false);
                }
            });

            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                close_delay as i32,
            ) {
                close_timeout.set_value(Some(id));
            }

            callback.forget();
        }
    };

    // Mouse handlers to keep hover card open when pointer is over content
    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(true);
        clear_close_timeout();
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        is_pointer_inside.set(false);
        schedule_close();
    };

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };
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
                class=class_val.get_value()
                data-radix-hover-card-content=""
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
pub fn HoverCardArrow(
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
