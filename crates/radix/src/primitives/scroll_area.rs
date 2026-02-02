use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{use_element_hover, use_scroll_with_options, UseScrollOptions};
use wasm_bindgen::JsCast;

/// Scroll visibility behavior
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ScrollType {
    /// Show when content overflows (default)
    #[default]
    Auto,
    /// Always show scrollbar
    Always,
    /// Show when scrolling
    Scroll,
    /// Show when hovering over scroll area
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
    // Scroll state
    scroll_x: Signal<f64>,
    scroll_y: Signal<f64>,
    is_scrolling: Signal<bool>,
    // Dimensions (need to track separately)
    scroll_height: RwSignal<f64>,
    scroll_width: RwSignal<f64>,
    client_height: RwSignal<f64>,
    client_width: RwSignal<f64>,
    // Visibility state
    is_hovering: Signal<bool>,
}

/// Context for scrollbar to share with thumb
#[derive(Clone, Copy)]
struct ScrollbarContext {
    orientation: Signal<Orientation>,
}

/// Root container for the scroll area. Provides context and manages visibility state.
#[component]
pub fn ScrollAreaRoot(
    /// Scrollbar visibility behavior. Default is `Auto`.
    #[prop(default = ScrollType::Auto.into(), into)]
    scroll_type: Signal<ScrollType>,

    /// Delay in milliseconds before hiding scrollbar after scrolling stops. Default is 600ms.
    #[prop(default = 600.into(), into)]
    scroll_hide_delay: Signal<u64>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The content to render within the scroll area.
    children: ChildrenFn,
) -> impl IntoView {
    // Create a separate ref for the viewport (the actual scrollable element)
    let viewport_ref = NodeRef::<Div>::new();

    // Use leptos-use's use_scroll for scroll position tracking
    // The idle option controls how long after scrolling stops before is_scrolling becomes false
    let scroll_return = use_scroll_with_options(
        viewport_ref,
        UseScrollOptions::default().idle(scroll_hide_delay.get_untracked() as f64),
    );
    let scroll_x = scroll_return.x;
    let scroll_y = scroll_return.y;
    let is_scrolling = scroll_return.is_scrolling;

    // Hover detection on the root element
    let is_hovering = use_element_hover(node_ref);

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

    let computed_style = move || {
        let base = "position: relative; overflow: hidden; width: 100%; height: 100%";
        match &style {
            Some(s) => format!("{}; {}", base, s),
            None => base.to_string(),
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=computed_style
            data-radix-scroll-area=""
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
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the viewport element.
    #[prop(optional, into)]
    style: Option<String>,

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

    let computed_style = move || {
        let base = "overflow: scroll; scrollbar-width: none; width: 100%; height: 100%";
        match &style {
            Some(s) => format!("{}; {}", base, s),
            None => base.to_string(),
        }
    };

    view! {
        <div
            node_ref=viewport_ref
            class=class
            style=computed_style
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

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the scrollbar element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the scrollbar element.
    #[prop(optional)]
    _node_ref: NodeRef<Div>,

    /// The scrollbar thumb.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ScrollAreaContext>()
        .expect("ScrollAreaScrollbar must be used within ScrollAreaRoot");

    // Create our own ref for track click handling
    let scrollbar_ref = NodeRef::<Div>::new();

    provide_context(ScrollbarContext { orientation });

    let orientation_attr = move || match orientation.get() {
        Orientation::Vertical => "vertical",
        Orientation::Horizontal => "horizontal",
    };

    // Compute visibility directly in closures to avoid signal timing issues
    let compute_visible = move || {
        if force_mount.get() {
            return true;
        }

        let has_overflow = match orientation.get() {
            Orientation::Vertical => ctx.scroll_height.get() > ctx.client_height.get(),
            Orientation::Horizontal => ctx.scroll_width.get() > ctx.client_width.get(),
        };

        match ctx.scroll_type.get() {
            ScrollType::Always => true,
            ScrollType::Auto => has_overflow,
            ScrollType::Scroll => ctx.is_scrolling.get(),
            ScrollType::Hover => ctx.is_hovering.get(),
        }
    };

    let state_attr = move || {
        if compute_visible() {
            "visible"
        } else {
            "hidden"
        }
    };

    // Handle click on track to jump to position
    let on_track_click = move |ev: web_sys::PointerEvent| {
        // Don't handle if clicking on the thumb (it will handle its own events)
        if let Some(target) = ev.target() {
            if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                if el.get_attribute("data-radix-scroll-area-thumb").is_some() {
                    return;
                }
            }
        }

        let Some(scrollbar_el) = scrollbar_ref.get() else {
            return;
        };
        let Some(viewport) = ctx.viewport_ref.get() else {
            return;
        };

        let rect = scrollbar_el.get_bounding_client_rect();
        let ori = orientation.get();

        // Calculate click position as percentage of track
        let click_percent = match ori {
            Orientation::Vertical => {
                let y = ev.client_y() as f64 - rect.top();
                (y / rect.height()).clamp(0.0, 1.0)
            }
            Orientation::Horizontal => {
                let x = ev.client_x() as f64 - rect.left();
                (x / rect.width()).clamp(0.0, 1.0)
            }
        };

        // Convert to scroll position
        let (client, scroll) = match ori {
            Orientation::Vertical => (ctx.client_height.get(), ctx.scroll_height.get()),
            Orientation::Horizontal => (ctx.client_width.get(), ctx.scroll_width.get()),
        };

        let max_scroll = scroll - client;
        if max_scroll <= 0.0 {
            return;
        }

        // Center the thumb at the click position
        let thumb_ratio = client / scroll;
        let adjusted_percent = (click_percent - thumb_ratio / 2.0).clamp(0.0, 1.0 - thumb_ratio);
        let new_scroll = adjusted_percent / (1.0 - thumb_ratio) * max_scroll;

        match ori {
            Orientation::Vertical => viewport.set_scroll_top(new_scroll as i32),
            Orientation::Horizontal => viewport.set_scroll_left(new_scroll as i32),
        }
    };

    let style_stored = StoredValue::new(style);
    let computed_style = move || {
        let display = if compute_visible() { "flex" } else { "none" };
        // Position scrollbar based on orientation
        let position_styles = match orientation.get() {
            Orientation::Vertical => "top: 0; right: 0; bottom: 0",
            Orientation::Horizontal => "left: 0; right: 0; bottom: 0",
        };
        let base = format!(
            "position: absolute; display: {}; {}; user-select: none; touch-action: none",
            display, position_styles
        );
        match style_stored.get_value() {
            Some(s) => format!("{}; {}", base, s),
            None => base,
        }
    };

    view! {
        <div
            node_ref=scrollbar_ref
            class=class
            style=computed_style
            data-radix-scroll-area-scrollbar=""
            data-orientation=orientation_attr
            data-state=state_attr
            on:pointerdown=on_track_click
        >
            {children()}
        </div>
    }
}

/// Draggable scrollbar thumb.
#[component]
pub fn ScrollAreaThumb(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the thumb element.
    #[prop(optional, into)]
    style: Option<String>,

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
        ev.stop_propagation(); // Prevent track click handler from firing

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

    let style_stored = StoredValue::new(style);
    let computed_style = move || {
        let size = thumb_size.get();
        let pos = thumb_position.get();

        // Use absolute positioning with insets to account for scrollbar padding (2px)
        // left/right: 2px centers thumb horizontally within the padding
        // top/bottom positioning uses percentages for the scroll position
        let base = match orientation.get() {
            Orientation::Vertical => format!(
                "position: absolute; left: 2px; right: 2px; top: {}%; height: {}%",
                pos, size
            ),
            Orientation::Horizontal => format!(
                "position: absolute; top: 2px; bottom: 2px; left: {}%; width: {}%",
                pos, size
            ),
        };
        match style_stored.get_value() {
            Some(s) => format!("{}; {}", base, s),
            None => base,
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=computed_style
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
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the corner element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the corner element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let computed_style = move || {
        let base = "position: absolute; right: 0; bottom: 0";
        match &style {
            Some(s) => format!("{}; {}", base, s),
            None => base.to_string(),
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=computed_style
            data-radix-scroll-area-corner=""
        />
    }
}
