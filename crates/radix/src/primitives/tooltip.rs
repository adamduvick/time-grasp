use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// Global timestamp of when any tooltip was last open (for skip delay behavior)
static LAST_TOOLTIP_CLOSE_TIME: AtomicU64 = AtomicU64::new(0);

fn current_time_ms() -> u64 {
    js_sys::Date::now() as u64
}

/// Side for tooltip positioning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    fn as_str(&self) -> &'static str {
        match self {
            TooltipSide::Top => "top",
            TooltipSide::Right => "right",
            TooltipSide::Bottom => "bottom",
            TooltipSide::Left => "left",
        }
    }
}

/// Alignment for tooltip positioning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipAlign {
    Start,
    #[default]
    Center,
    End,
}

impl TooltipAlign {
    fn as_str(&self) -> &'static str {
        match self {
            TooltipAlign::Start => "start",
            TooltipAlign::Center => "center",
            TooltipAlign::End => "end",
        }
    }
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
    trigger_rect: RwSignal<Option<TriggerRect>>,
    delay_duration: u32,
    skip_delay_duration: u32,
    /// Tracks whether the tooltip opened with a delay or instantly
    was_instant: RwSignal<bool>,
    /// Current side for arrow positioning
    current_side: RwSignal<TooltipSide>,
    /// Whether pointer is currently over trigger or content (grace period for moving between them)
    is_pointer_inside: RwSignal<bool>,
    /// Arrow height for offset calculation (set by TooltipArrow)
    arrow_height: RwSignal<u32>,
}

#[derive(Clone, Copy, Default)]
struct TriggerRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
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
    let trigger_rect = RwSignal::new(None);
    let was_instant = RwSignal::new(false);
    let current_side = RwSignal::new(TooltipSide::Top);
    let is_pointer_inside = RwSignal::new(false);
    let arrow_height = RwSignal::new(5); // Default arrow height

    let ctx = TooltipContext {
        open: open_signal,
        trigger_rect,
        delay_duration: delay,
        skip_delay_duration: skip_delay,
        was_instant,
        current_side,
        is_pointer_inside,
        arrow_height,
    };

    provide_context(ctx);

    children()
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

    /// Reference to the trigger element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,

    /// The trigger content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipTrigger must be used within TooltipRoot");

    let trigger_ref: NodeRef<leptos::html::Span> = node_ref;
    let hover_timeout: StoredValue<Option<i32>> = StoredValue::new(None);

    let update_trigger_rect = move || {
        if let Some(el) = trigger_ref.get() {
            let html_el: &web_sys::HtmlElement = &el;
            let rect = html_el.get_bounding_client_rect();
            ctx.trigger_rect.set(Some(TriggerRect {
                x: rect.x(),
                y: rect.y(),
                width: rect.width(),
                height: rect.height(),
            }));
        }
    };

    let clear_timeout = move || {
        if let Some(id) = hover_timeout.get_value() {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
            hover_timeout.set_value(None);
        }
    };

    let open_tooltip = move |instant: bool| {
        update_trigger_rect();
        ctx.was_instant.set(instant);
        ctx.open.set(true);
    };

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
        // Small grace period to allow moving to content
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                // Only close if pointer is not inside trigger or content
                if !ctx.is_pointer_inside.get() {
                    if ctx.open.get() {
                        LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
                    }
                    ctx.open.set(false);
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
        ctx.is_pointer_inside.set(true);

        // Check if we should skip the delay (recently viewed another tooltip)
        let last_close = LAST_TOOLTIP_CLOSE_TIME.load(Ordering::Relaxed);
        let now = current_time_ms();
        let skip_delay = now.saturating_sub(last_close) < ctx.skip_delay_duration as u64;

        if ctx.delay_duration == 0 || skip_delay {
            open_tooltip(true);
        } else {
            // Set up delayed open
            if let Some(window) = web_sys::window() {
                let callback = Closure::<dyn Fn()>::new(move || {
                    open_tooltip(false);
                });

                if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    ctx.delay_duration as i32,
                ) {
                    hover_timeout.set_value(Some(id));
                }

                callback.forget();
            }
        }
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        clear_timeout();
        ctx.is_pointer_inside.set(false);
        // Update timestamp immediately when leaving an open tooltip (for skip delay behavior)
        // This ensures moving quickly between tooltips works, even before the grace period closes the tooltip
        if ctx.open.get() {
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
        if ctx.open.get() {
            LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
        }
        ctx.open.set(false);
    };

    // Handle Escape key to close tooltip
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" && ctx.open.get() {
            ev.prevent_default();
            LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
            ctx.open.set(false);
        }
    };

    let state_attr = move || {
        if ctx.open.get() {
            if ctx.was_instant.get() {
                "instant-open"
            } else {
                "delayed-open"
            }
        } else {
            "closed"
        }
    };

    view! {
        <span
            node_ref=trigger_ref
            class=class
            style=style
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
    }
}

/// Portal container - renders tooltip to document.body when open.
#[component]
pub fn TooltipPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipPortal must be used within TooltipRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <Portal>
                <TooltipPortalInner ctx=ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn TooltipPortalInner(ctx: TooltipContext, children: StoredValue<ChildrenFn>) -> impl IntoView {
    provide_context(ctx);
    children.with_value(|c| c())
}

/// The tooltip content.
#[component]
pub fn TooltipContent(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Which side of the trigger to show the tooltip.
    #[prop(default = TooltipSide::Top)]
    side: TooltipSide,

    /// Alignment along the side.
    #[prop(default = TooltipAlign::Center)]
    align: TooltipAlign,

    /// Offset from the trigger in pixels.
    #[prop(default = 0)]
    side_offset: i32,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The tooltip content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipContent must be used within TooltipRoot");

    // Store the side in context so arrow can access it
    ctx.current_side.set(side);

    let content_ref: NodeRef<leptos::html::Div> = node_ref;

    // Mouse handlers to keep tooltip open when hovering over content
    let on_mouse_enter = move |_: web_sys::MouseEvent| {
        ctx.is_pointer_inside.set(true);
    };

    let on_mouse_leave = move |_: web_sys::MouseEvent| {
        ctx.is_pointer_inside.set(false);
        // Schedule close with grace period
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                if !ctx.is_pointer_inside.get() {
                    if ctx.open.get() {
                        LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
                    }
                    ctx.open.set(false);
                }
            });

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100, // 100ms grace period
            );

            callback.forget();
        }
    };

    // Calculate position based on trigger rect and side
    let position_style = move || {
        let Some(rect) = ctx.trigger_rect.get() else {
            return String::new();
        };

        // Get scroll offsets
        let (scroll_x, scroll_y) = web_sys::window()
            .map(|w| (w.scroll_x().unwrap_or(0.0), w.scroll_y().unwrap_or(0.0)))
            .unwrap_or((0.0, 0.0));

        let trigger_center_x = rect.x + rect.width / 2.0 + scroll_x;
        let trigger_center_y = rect.y + rect.height / 2.0 + scroll_y;
        // Include arrow height in offset (like React Radix does)
        let arrow_h = ctx.arrow_height.get() as f64;
        let offset = side_offset as f64 + arrow_h;

        let (left, top, transform) = match side {
            TooltipSide::Top => {
                let left = trigger_center_x;
                let top = rect.y + scroll_y - offset;
                let transform = match align {
                    TooltipAlign::Start => "translateX(0) translateY(-100%)",
                    TooltipAlign::Center => "translateX(-50%) translateY(-100%)",
                    TooltipAlign::End => "translateX(-100%) translateY(-100%)",
                };
                let left = match align {
                    TooltipAlign::Start => rect.x + scroll_x,
                    TooltipAlign::Center => left,
                    TooltipAlign::End => rect.x + rect.width + scroll_x,
                };
                (left, top, transform)
            }
            TooltipSide::Bottom => {
                let left = trigger_center_x;
                let top = rect.y + rect.height + scroll_y + offset;
                let transform = match align {
                    TooltipAlign::Start => "translateX(0)",
                    TooltipAlign::Center => "translateX(-50%)",
                    TooltipAlign::End => "translateX(-100%)",
                };
                let left = match align {
                    TooltipAlign::Start => rect.x + scroll_x,
                    TooltipAlign::Center => left,
                    TooltipAlign::End => rect.x + rect.width + scroll_x,
                };
                (left, top, transform)
            }
            TooltipSide::Left => {
                let left = rect.x + scroll_x - offset;
                let top = trigger_center_y;
                let transform = match align {
                    TooltipAlign::Start => "translateX(-100%) translateY(0)",
                    TooltipAlign::Center => "translateX(-100%) translateY(-50%)",
                    TooltipAlign::End => "translateX(-100%) translateY(-100%)",
                };
                let top = match align {
                    TooltipAlign::Start => rect.y + scroll_y,
                    TooltipAlign::Center => top,
                    TooltipAlign::End => rect.y + rect.height + scroll_y,
                };
                (left, top, transform)
            }
            TooltipSide::Right => {
                let left = rect.x + rect.width + scroll_x + offset;
                let top = trigger_center_y;
                let transform = match align {
                    TooltipAlign::Start => "translateY(0)",
                    TooltipAlign::Center => "translateY(-50%)",
                    TooltipAlign::End => "translateY(-100%)",
                };
                let top = match align {
                    TooltipAlign::Start => rect.y + scroll_y,
                    TooltipAlign::Center => top,
                    TooltipAlign::End => rect.y + rect.height + scroll_y,
                };
                (left, top, transform)
            }
        };

        format!(
            "position: absolute; left: {:.0}px; top: {:.0}px; transform: {}; z-index: 50;",
            left, top, transform
        )
    };

    let combined_style = move || {
        let pos = position_style();
        match &style {
            Some(s) => format!("{} {}", pos, s),
            None => pos,
        }
    };

    let state_attr = move || {
        if ctx.open.get() {
            if ctx.was_instant.get() {
                "instant-open"
            } else {
                "delayed-open"
            }
        } else {
            "closed"
        }
    };

    // Close on mouse enter of content (to handle edge cases)
    let on_pointer_down_outside = move |_: web_sys::MouseEvent| {
        LAST_TOOLTIP_CLOSE_TIME.store(current_time_ms(), Ordering::Relaxed);
        ctx.open.set(false);
    };

    view! {
        <div
            node_ref=content_ref
            role="tooltip"
            class=class
            style=combined_style
            data-radix-tooltip-content=""
            data-state=state_attr
            data-side=side.as_str()
            data-align=align.as_str()
            on:mouseenter=on_mouse_enter
            on:mouseleave=on_mouse_leave
            on:pointerdownoutside=on_pointer_down_outside
        >
            {children()}
        </div>
    }
}

/// Arrow component that points to the trigger.
#[component]
pub fn TooltipArrow(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Width of the arrow in pixels.
    #[prop(default = 10)]
    width: u32,

    /// Height of the arrow in pixels.
    #[prop(default = 5)]
    height: u32,
) -> impl IntoView {
    let ctx =
        use_context::<TooltipContext>().expect("TooltipArrow must be used within TooltipRoot");

    // Set arrow height in context for offset calculation
    ctx.arrow_height.set(height);

    // Arrow rotation and positioning based on tooltip side
    // The arrow is anchored to the edge and uses transform to extend outside the content box
    // This matches React Radix behavior where the arrow extends from the tooltip edge
    // Note: The arrow SVG points DOWN by default (like React Radix), so:
    // - Top: no rotation (points down toward trigger)
    // - Bottom: 180deg (points up toward trigger)
    // - Left: -90deg (points right toward trigger)
    // - Right: 90deg (points left toward trigger)
    // For left/right, we use the arrow height for translation since after rotation
    // the visual protrusion is the height, not the width
    let arrow_style = move || {
        let side = ctx.current_side.get();
        match side {
            // Tooltip above trigger -> arrow at bottom pointing down (no rotation)
            TooltipSide::Top => {
                "position: absolute; bottom: 0; left: 50%; transform: translateX(-50%) translateY(100%);".to_string()
            }
            // Tooltip below trigger -> arrow at top pointing up (rotate 180deg)
            TooltipSide::Bottom => {
                "position: absolute; top: 0; left: 50%; transform: translateX(-50%) translateY(-100%) rotate(180deg);".to_string()
            }
            // Tooltip left of trigger -> arrow at right pointing right (rotate -90deg)
            // Use height for X translation since that's the visual protrusion after rotation
            TooltipSide::Left => {
                format!("position: absolute; right: 0; top: 50%; transform: translateY(-50%) translateX({}px) rotate(-90deg);", height)
            }
            // Tooltip right of trigger -> arrow at left pointing left (rotate 90deg)
            // Use height for X translation since that's the visual protrusion after rotation
            TooltipSide::Right => {
                format!("position: absolute; left: 0; top: 50%; transform: translateY(-50%) translateX(-{}px) rotate(90deg);", height)
            }
        }
    };

    // Arrow shape: flat top edge, tip at bottom (like React Radix "0,0 30,0 15,10")
    let points = format!("0,0 {},0 {},{}", width, width / 2, height);

    view! {
        <svg
            class=class
            width=width
            height=height
            viewBox=format!("0 0 {} {}", width, height)
            preserveAspectRatio="none"
            data-radix-tooltip-arrow=""
            style=arrow_style
        >
            <polygon points=points />
        </svg>
    }
}
