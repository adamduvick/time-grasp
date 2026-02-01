use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{UseScrollOptions, UseScrollReturn, use_scroll_with_options};
use wasm_bindgen::JsCast;

/// Scroll visibility behavior
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ScrollType {
    /// Show when hovering AND content overflows
    Auto,
    /// Always show scrollbar
    Always,
    /// Show when scrolling
    Scroll,
    /// Show when hovering over scroll area
    #[default]
    Hover,
}

/// Scrollbar orientation
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Context shared between ScrollArea components
#[derive(Clone, Copy)]
struct ScrollAreaContext {
    scroll_type: Signal<ScrollType>,
    viewport_ref: NodeRef<Div>,
    // Scroll state from use_scroll
    scroll_x: Signal<f64>,
    scroll_y: Signal<f64>,
    is_scrolling: Signal<bool>,
    // Dimensions (need to track separately)
    scroll_height: RwSignal<f64>,
    scroll_width: RwSignal<f64>,
    client_height: RwSignal<f64>,
    client_width: RwSignal<f64>,
    // Visibility state
    is_hovering: RwSignal<bool>,
}

/// Context for scrollbar to share with thumb
#[derive(Clone, Copy)]
struct ScrollbarContext {
    orientation: Signal<Orientation>,
}

/// Root container for the scroll area. Provides context and manages visibility state.
#[component]
pub fn ScrollAreaRoot(
    /// Scrollbar visibility behavior. Default is `Hover`.
    #[prop(default = ScrollType::Hover.into(), into)]
    scroll_type: Signal<ScrollType>,

    /// Delay in milliseconds before hiding scrollbar after scrolling stops. Default is 600ms.
    #[prop(default = 600.into(), into)]
    scroll_hide_delay: Signal<u64>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The content to render within the scroll area.
    children: ChildrenFn,
) -> impl IntoView {
    let viewport_ref = NodeRef::<Div>::new();

    // Use leptos-use's use_scroll for scroll state tracking
    let UseScrollReturn {
        x: scroll_x,
        y: scroll_y,
        is_scrolling,
        ..
    } = use_scroll_with_options(
        viewport_ref,
        UseScrollOptions::default().idle(scroll_hide_delay.get_untracked() as f64),
    );

    let is_hovering = RwSignal::new(false);

    let ctx = ScrollAreaContext {
        scroll_type,
        viewport_ref,
        scroll_x,
        scroll_y,
        is_scrolling,
        scroll_height: RwSignal::new(0.0),
        scroll_width: RwSignal::new(0.0),
        client_height: RwSignal::new(0.0),
        client_width: RwSignal::new(0.0),
        is_hovering,
    };

    provide_context(ctx);

    view! {
        <div
            node_ref=node_ref
            style:position="relative"
            style:overflow="hidden"
            style:width="100%"
            style:height="100%"
            data-radix-scroll-area=""
            on:mouseenter=move |_| is_hovering.set(true)
            on:mouseleave=move |_| is_hovering.set(false)
        >
            {children()}
        </div>
    }
}

/// Scrollable content area. Hides native scrollbars.
///
/// Note: Users should add this CSS to hide webkit scrollbars:
/// ```css
/// [data-radix-scroll-area-viewport]::-webkit-scrollbar {
///     display: none;
/// }
/// ```
#[component]
pub fn ScrollAreaViewport(
    /// Reference to the viewport element.
    #[prop(optional)]
    _node_ref: NodeRef<Div>,

    /// The scrollable content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ScrollAreaContext>()
        .expect("ScrollAreaViewport must be used within ScrollAreaRoot");

    // Always use the context viewport_ref since use_scroll is bound to it
    let viewport_ref = ctx.viewport_ref;

    // Update dimensions on mount and when content changes
    Effect::new(move |_| {
        let Some(el) = viewport_ref.get() else {
            return;
        };

        ctx.scroll_height.set(el.scroll_height() as f64);
        ctx.scroll_width.set(el.scroll_width() as f64);
        ctx.client_height.set(el.client_height() as f64);
        ctx.client_width.set(el.client_width() as f64);
    });

    // Also update dimensions on scroll (content size might change)
    Effect::new(move |_| {
        // Track scroll position to trigger re-evaluation
        let _ = ctx.scroll_x.get();
        let _ = ctx.scroll_y.get();

        let Some(el) = viewport_ref.get() else {
            return;
        };

        ctx.scroll_height.set(el.scroll_height() as f64);
        ctx.scroll_width.set(el.scroll_width() as f64);
        ctx.client_height.set(el.client_height() as f64);
        ctx.client_width.set(el.client_width() as f64);
    });

    view! {
        <div
            node_ref=viewport_ref
            style:overflow="scroll"
            style:scrollbar-width="none"
            style:width="100%"
            style:height="100%"
            data-radix-scroll-area-viewport=""
        >
            {children()}
        </div>
    }
}

/// Scrollbar track (vertical or horizontal).
#[component]
pub fn ScrollAreaScrollbar(
    /// Scrollbar orientation. Default is `Vertical`.
    #[prop(default = Orientation::Vertical.into(), into)]
    orientation: Signal<Orientation>,

    /// Force the scrollbar to always be visible.
    #[prop(default = false.into(), into)]
    force_mount: Signal<bool>,

    /// Reference to the scrollbar element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The scrollbar thumb.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ScrollAreaContext>()
        .expect("ScrollAreaScrollbar must be used within ScrollAreaRoot");

    provide_context(ScrollbarContext { orientation });

    // Determine if scrollbar has overflow content
    let has_overflow = Signal::derive(move || {
        let ori = orientation.get();
        match ori {
            Orientation::Vertical => ctx.scroll_height.get() > ctx.client_height.get(),
            Orientation::Horizontal => ctx.scroll_width.get() > ctx.client_width.get(),
        }
    });

    // Handle visibility based on scroll_type
    let is_visible = Signal::derive(move || {
        if force_mount.get() {
            return true;
        }

        match ctx.scroll_type.get() {
            ScrollType::Always => true,
            ScrollType::Auto => ctx.is_hovering.get() && has_overflow.get(),
            ScrollType::Scroll => ctx.is_scrolling.get(),
            ScrollType::Hover => ctx.is_hovering.get(),
        }
    });

    let orientation_attr = move || match orientation.get() {
        Orientation::Vertical => "vertical",
        Orientation::Horizontal => "horizontal",
    };

    let state_attr = move || {
        if is_visible.get() {
            "visible"
        } else {
            "hidden"
        }
    };

    view! {
        <div
            node_ref=node_ref
            style:position="absolute"
            style:display=move || if is_visible.get() { "block" } else { "none" }
            style:user-select="none"
            style:touch-action="none"
            data-radix-scroll-area-scrollbar=""
            data-orientation=orientation_attr
            data-state=state_attr
        >
            {children()}
        </div>
    }
}

/// Draggable scrollbar thumb.
#[component]
pub fn ScrollAreaThumb(
    /// Reference to the thumb element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx = use_context::<ScrollAreaContext>()
        .expect("ScrollAreaThumb must be used within ScrollAreaRoot");

    let scrollbar_ctx = use_context::<ScrollbarContext>()
        .expect("ScrollAreaThumb must be used within ScrollAreaScrollbar");

    let orientation = scrollbar_ctx.orientation;

    // Track drag state
    let is_dragging = RwSignal::new(false);
    let drag_start_pos = RwSignal::new(0.0);
    let drag_start_scroll = RwSignal::new(0.0);

    // Calculate thumb size as percentage of track
    let thumb_size = Signal::derive(move || {
        let ori = orientation.get();
        let (client, scroll) = match ori {
            Orientation::Vertical => (ctx.client_height.get(), ctx.scroll_height.get()),
            Orientation::Horizontal => (ctx.client_width.get(), ctx.scroll_width.get()),
        };

        if scroll <= 0.0 {
            return 100.0;
        }

        (client / scroll * 100.0).min(100.0)
    });

    // Calculate thumb position as percentage offset
    let thumb_position = Signal::derive(move || {
        let ori = orientation.get();
        let (scroll_pos, client, scroll) = match ori {
            Orientation::Vertical => (
                ctx.scroll_y.get(),
                ctx.client_height.get(),
                ctx.scroll_height.get(),
            ),
            Orientation::Horizontal => (
                ctx.scroll_x.get(),
                ctx.client_width.get(),
                ctx.scroll_width.get(),
            ),
        };

        let max_scroll = scroll - client;
        if max_scroll <= 0.0 {
            return 0.0;
        }

        let scroll_ratio = scroll_pos / max_scroll;
        scroll_ratio * (100.0 - thumb_size.get())
    });

    // Pointer event handlers for drag
    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        ev.prevent_default();

        if let Some(target) = ev.target()
            && let Ok(el) = target.dyn_into::<web_sys::Element>()
        {
            _ = el.set_pointer_capture(ev.pointer_id());
        }

        is_dragging.set(true);

        let ori = orientation.get();
        let (pos, scroll) = match ori {
            Orientation::Vertical => (ev.client_y() as f64, ctx.scroll_y.get()),
            Orientation::Horizontal => (ev.client_x() as f64, ctx.scroll_x.get()),
        };

        drag_start_pos.set(pos);
        drag_start_scroll.set(scroll);
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        if !is_dragging.get() {
            return;
        }

        ev.prevent_default();

        let Some(viewport) = ctx.viewport_ref.get() else {
            return;
        };

        let ori = orientation.get();
        let (current_pos, client, scroll) = match ori {
            Orientation::Vertical => (
                ev.client_y() as f64,
                ctx.client_height.get(),
                ctx.scroll_height.get(),
            ),
            Orientation::Horizontal => (
                ev.client_x() as f64,
                ctx.client_width.get(),
                ctx.scroll_width.get(),
            ),
        };

        let delta = current_pos - drag_start_pos.get();

        // Convert thumb delta to scroll delta
        // The thumb moves within (track_size - thumb_size), which maps to (scroll - client)
        let track_size = client;
        let thumb_size_px = thumb_size.get() / 100.0 * track_size;
        let scrollable_track = track_size - thumb_size_px;
        let max_scroll = scroll - client;

        if scrollable_track <= 0.0 {
            return;
        }

        let scroll_delta = delta * (max_scroll / scrollable_track);
        let new_scroll = (drag_start_scroll.get() + scroll_delta).clamp(0.0, max_scroll);

        match ori {
            Orientation::Vertical => viewport.set_scroll_top(new_scroll as i32),
            Orientation::Horizontal => viewport.set_scroll_left(new_scroll as i32),
        }
    };

    let on_pointer_up = move |ev: web_sys::PointerEvent| {
        if let Some(target) = ev.target()
            && let Ok(el) = target.dyn_into::<web_sys::Element>()
        {
            _ = el.release_pointer_capture(ev.pointer_id());
        }

        is_dragging.set(false);
    };

    let state_attr = move || {
        if is_dragging.get() {
            "dragging"
        } else {
            "idle"
        }
    };

    view! {
        <div
            node_ref=node_ref
            style:position="absolute"
            style:width=move || {
                match orientation.get() {
                    Orientation::Vertical => "100%".to_string(),
                    Orientation::Horizontal => format!("{}%", thumb_size.get()),
                }
            }
            style:height=move || {
                match orientation.get() {
                    Orientation::Vertical => format!("{}%", thumb_size.get()),
                    Orientation::Horizontal => "100%".to_string(),
                }
            }
            style:top=move || {
                match orientation.get() {
                    Orientation::Vertical => format!("{}%", thumb_position.get()),
                    Orientation::Horizontal => "0".to_string(),
                }
            }
            style:left=move || {
                match orientation.get() {
                    Orientation::Vertical => "0".to_string(),
                    Orientation::Horizontal => format!("{}%", thumb_position.get()),
                }
            }
            data-radix-scroll-area-thumb=""
            data-state=state_attr
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
        />
    }
}

/// Corner element where vertical and horizontal scrollbars meet.
#[component]
pub fn ScrollAreaCorner(
    /// Reference to the corner element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            style:position="absolute"
            style:right="0"
            style:bottom="0"
            data-radix-scroll-area-corner=""
        />
    }
}
