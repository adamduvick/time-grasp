use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// Helper to get visual viewport dimensions
fn get_visual_viewport() -> Option<(f64, f64, f64, f64)> {
    let window = web_sys::window()?;
    let vv: JsValue = js_sys::Reflect::get(&window, &JsValue::from_str("visualViewport")).ok()?;

    if vv.is_undefined() || vv.is_null() {
        return None;
    }

    let width = js_sys::Reflect::get(&vv, &JsValue::from_str("width"))
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let height = js_sys::Reflect::get(&vv, &JsValue::from_str("height"))
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let offset_left = js_sys::Reflect::get(&vv, &JsValue::from_str("offsetLeft"))
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let offset_top = js_sys::Reflect::get(&vv, &JsValue::from_str("offsetTop"))
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    Some((width, height, offset_left, offset_top))
}

/// Helper to add event listener to visual viewport
fn add_visual_viewport_listener(callback: &Closure<dyn Fn()>) {
    if let Some(window) = web_sys::window() {
        if let Ok(vv) = js_sys::Reflect::get(&window, &JsValue::from_str("visualViewport")) {
            if !vv.is_undefined() && !vv.is_null() {
                // Add resize listener
                let _ = js_sys::Reflect::get(&vv, &JsValue::from_str("addEventListener"))
                    .ok()
                    .and_then(|add_fn| {
                        let add_fn: js_sys::Function = add_fn.dyn_into().ok()?;
                        add_fn.call2(&vv, &JsValue::from_str("resize"), callback.as_ref().unchecked_ref()).ok()
                    });
                // Add scroll listener
                let _ = js_sys::Reflect::get(&vv, &JsValue::from_str("addEventListener"))
                    .ok()
                    .and_then(|add_fn| {
                        let add_fn: js_sys::Function = add_fn.dyn_into().ok()?;
                        add_fn.call2(&vv, &JsValue::from_str("scroll"), callback.as_ref().unchecked_ref()).ok()
                    });
            }
        }
    }
}

/// Side options for popper positioning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopperSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl PopperSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopperSide::Top => "top",
            PopperSide::Right => "right",
            PopperSide::Bottom => "bottom",
            PopperSide::Left => "left",
        }
    }

    fn opposite(&self) -> PopperSide {
        match self {
            PopperSide::Top => PopperSide::Bottom,
            PopperSide::Bottom => PopperSide::Top,
            PopperSide::Left => PopperSide::Right,
            PopperSide::Right => PopperSide::Left,
        }
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, PopperSide::Left | PopperSide::Right)
    }
}

/// Alignment options for popper positioning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopperAlign {
    Start,
    #[default]
    Center,
    End,
}

impl PopperAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopperAlign::Start => "start",
            PopperAlign::Center => "center",
            PopperAlign::End => "end",
        }
    }
}

/// Rectangle measurements for positioning.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// Positioning data calculated for each update.
#[derive(Clone, Copy, Default, Debug)]
struct PositionData {
    x: f64,
    y: f64,
    anchor_width: f64,
    anchor_height: f64,
    content_width: f64,
    content_height: f64,
    available_width: f64,
    available_height: f64,
    placed_side: PopperSide,
    placed_align: PopperAlign,
    /// Arrow position along the alignment axis (in pixels from start)
    arrow_pos: f64,
    /// Transform origin X
    origin_x: f64,
    /// Transform origin Y
    origin_y: f64,
}

/// Context shared between Popper components.
#[derive(Clone, Copy)]
struct PopperContext {
    /// NodeRef to the anchor element for measurements
    anchor_ref: NodeRef<leptos::html::Div>,
    /// Signal to trigger position updates
    update_trigger: RwSignal<u32>,
}

/// Context shared between PopperContent and PopperArrow.
#[derive(Clone, Copy)]
struct PopperContentContext {
    /// The actual placed side (may differ from requested if flipped)
    placed_side: RwSignal<PopperSide>,
    /// The actual placed alignment
    placed_align: RwSignal<PopperAlign>,
    /// Arrow position along the alignment axis (pixels from edge)
    arrow_pos: RwSignal<f64>,
    /// Arrow width for positioning
    arrow_width: RwSignal<u32>,
    /// Arrow height for offset calculation
    arrow_height: RwSignal<u32>,
    /// Whether to hide arrow (can't fit it)
    should_hide_arrow: RwSignal<bool>,
}

/// Root container for the Popper.
#[component]
pub fn PopperRoot(
    /// The popper parts.
    children: Children,
) -> impl IntoView {
    let anchor_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let update_trigger = RwSignal::new(0u32);

    let ctx = PopperContext {
        anchor_ref,
        update_trigger,
    };

    provide_context(ctx);

    children()
}

/// The anchor element to position relative to.
#[component]
pub fn PopperAnchor(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The anchor content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<PopperContext>().expect("PopperAnchor must be used within PopperRoot");

    // Trigger initial update when mounted
    Effect::new(move || {
        if ctx.anchor_ref.get().is_some() {
            ctx.update_trigger.set(ctx.update_trigger.get_untracked() + 1);
        }
    });

    view! {
        <div
            node_ref=ctx.anchor_ref
            class=class
            style=style
            data-radix-popper-anchor=""
        >
            {children()}
        </div>
    }
}

/// The positioned floating content.
#[component]
pub fn PopperContent(
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

    /// Inline styles (will be merged with positioning styles).
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// Callback when content is positioned.
    #[prop(optional)]
    on_placed: Option<Callback<()>>,

    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<PopperContext>().expect("PopperContent must be used within PopperRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let wrapper_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let is_positioned = RwSignal::new(false);

    // Create content context for arrow
    let placed_side = RwSignal::new(side.get_untracked());
    let placed_align = RwSignal::new(align.get_untracked());
    let arrow_pos = RwSignal::new(0.0f64);
    let arrow_width = RwSignal::new(10u32);
    // Initialize to 0 - only set when PopperArrow is rendered
    let arrow_height = RwSignal::new(0u32);
    let should_hide_arrow = RwSignal::new(false);

    let content_ctx = PopperContentContext {
        placed_side,
        placed_align,
        arrow_pos,
        arrow_width,
        arrow_height,
        should_hide_arrow,
    };

    provide_context(content_ctx);

    // Set up scroll and resize listeners
    Effect::new(move || {
        let Some(_content_el) = content_ref.get() else {
            return;
        };

        let Some(window) = web_sys::window() else {
            return;
        };

        let update_trigger = ctx.update_trigger;

        // Create update callback
        let update_callback = Rc::new(RefCell::new(None::<Closure<dyn Fn()>>));
        let update_callback_clone = update_callback.clone();

        let callback = Closure::<dyn Fn()>::new(move || {
            update_trigger.set(update_trigger.get_untracked().wrapping_add(1));
        });

        *update_callback_clone.borrow_mut() = Some(callback);

        // Add scroll listener (capture phase to catch all scroll events)
        if let Some(ref cb) = *update_callback.borrow() {
            let _ = window.add_event_listener_with_callback_and_bool(
                "scroll",
                cb.as_ref().unchecked_ref(),
                true, // capture phase
            );

            let _ = window.add_event_listener_with_callback(
                "resize",
                cb.as_ref().unchecked_ref(),
            );

            // Add visual viewport listeners for pinch-zoom support
            add_visual_viewport_listener(cb);
        }

        // Also set up requestAnimationFrame loop for smooth updates
        let raf_callback = Rc::new(RefCell::new(None::<Closure<dyn Fn()>>));
        let raf_callback_clone = raf_callback.clone();
        let raf_id = Rc::new(RefCell::new(0i32));
        let raf_id_clone = raf_id.clone();

        let raf_fn = Closure::<dyn Fn()>::new(move || {
            update_trigger.set(update_trigger.get_untracked().wrapping_add(1));

            // Schedule next frame
            if let Some(window) = web_sys::window() {
                if let Some(ref cb) = *raf_callback_clone.borrow() {
                    if let Ok(id) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                        *raf_id_clone.borrow_mut() = id;
                    }
                }
            }
        });

        *raf_callback.borrow_mut() = Some(raf_fn);

        // Start the RAF loop
        if let Some(ref cb) = *raf_callback.borrow() {
            if let Ok(id) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                *raf_id.borrow_mut() = id;
            }
        }

        // Leak the closures to keep them alive
        if let Some(cb) = update_callback.borrow_mut().take() {
            cb.forget();
        }
        if let Some(cb) = raf_callback.borrow_mut().take() {
            cb.forget();
        }
    });

    // Calculate position data
    let position_data = move || {
        // Subscribe to update trigger
        ctx.update_trigger.get();

        // Get anchor rect from NodeRef
        let anchor = ctx.anchor_ref.get().map(|el| {
            let html_el: &web_sys::HtmlElement = &el;
            let rect = html_el.get_bounding_client_rect();
            Rect {
                x: rect.x(),
                y: rect.y(),
                width: rect.width(),
                height: rect.height(),
            }
        });

        // Get content rect from wrapper (for accurate dimensions)
        let content = wrapper_ref.get().map(|el| {
            let html_el: &web_sys::HtmlElement = &el;
            let rect = html_el.get_bounding_client_rect();
            Rect {
                x: rect.x(),
                y: rect.y(),
                width: rect.width(),
                height: rect.height(),
            }
        });

        let Some(anchor) = anchor else {
            return None;
        };

        // Use estimated content size if not yet measured
        let content_width = content.map(|c| c.width).unwrap_or(100.0);
        let content_height = content.map(|c| c.height).unwrap_or(50.0);

        // Mark as positioned
        if !is_positioned.get_untracked() {
            is_positioned.set(true);
            if let Some(cb) = on_placed {
                cb.run(());
            }
        }

        // Get viewport dimensions using visualViewport API for pinch-zoom support
        // Falls back to window.innerWidth/Height if visualViewport not available
        let (viewport_width, viewport_height, viewport_offset_x, viewport_offset_y) =
            if let Some((w, h, ox, oy)) = get_visual_viewport() {
                (w, h, ox, oy)
            } else {
                // Fallback to layout viewport
                web_sys::window()
                    .map(|w| {
                        (
                            w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0),
                            w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0),
                            0.0,
                            0.0,
                        )
                    })
                    .unwrap_or((0.0, 0.0, 0.0, 0.0))
            };

        // NOTE: We do NOT adjust anchor coordinates for viewport offset.
        // getBoundingClientRect() returns coordinates relative to the layout viewport,
        // and position:fixed also positions relative to the layout viewport.
        // Adjusting would cause parallax during pinch-zoom.

        // Read reactive props
        let side_val = side.get();
        let side_offset_val = side_offset.get() as f64;
        let align_val = align.get();
        let align_offset_val = align_offset.get() as f64;
        let avoid_collisions_val = avoid_collisions.get();
        let collision_pad = collision_padding.get() as f64;

        // Arrow dimensions
        let arrow_h = arrow_height.get() as f64;
        let arrow_w = arrow_width.get() as f64;

        // Total offset includes arrow height
        let total_offset = side_offset_val + arrow_h;

        // Calculate visual viewport bounds in layout viewport coordinates.
        // Collision detection should check if content fits within the VISUAL viewport,
        // but positioning uses raw layout viewport coordinates.
        let vv_left = viewport_offset_x + collision_pad;
        let vv_top = viewport_offset_y + collision_pad;
        let vv_right = viewport_offset_x + viewport_width - collision_pad;
        let vv_bottom = viewport_offset_y + viewport_height - collision_pad;

        // Calculate space on each side relative to visual viewport bounds
        let space_top = anchor.y - vv_top;
        let space_bottom = vv_bottom - (anchor.y + anchor.height);
        let space_left = anchor.x - vv_left;
        let space_right = vv_right - (anchor.x + anchor.width);

        // Determine actual side (may flip if collision)
        let mut actual_side = side_val;
        if avoid_collisions_val {
            actual_side = match side_val {
                PopperSide::Top if space_top < content_height && space_bottom > space_top => {
                    PopperSide::Bottom
                }
                PopperSide::Bottom if space_bottom < content_height && space_top > space_bottom => {
                    PopperSide::Top
                }
                PopperSide::Left if space_left < content_width && space_right > space_left => {
                    PopperSide::Right
                }
                PopperSide::Right if space_right < content_width && space_left > space_right => {
                    PopperSide::Left
                }
                _ => side_val,
            };
        }

        // Calculate position using transform: translate() approach like React Radix
        // We position at left:0, top:0 and use transform to move
        let (x, y) = match actual_side {
            PopperSide::Top => {
                let y = anchor.y - content_height - total_offset;
                let x = match align_val {
                    PopperAlign::Start => anchor.x + align_offset_val,
                    PopperAlign::Center => anchor.x + (anchor.width - content_width) / 2.0 + align_offset_val,
                    PopperAlign::End => anchor.x + anchor.width - content_width + align_offset_val,
                };
                (x, y)
            }
            PopperSide::Bottom => {
                let y = anchor.y + anchor.height + total_offset;
                let x = match align_val {
                    PopperAlign::Start => anchor.x + align_offset_val,
                    PopperAlign::Center => anchor.x + (anchor.width - content_width) / 2.0 + align_offset_val,
                    PopperAlign::End => anchor.x + anchor.width - content_width + align_offset_val,
                };
                (x, y)
            }
            PopperSide::Left => {
                let x = anchor.x - content_width - total_offset;
                let y = match align_val {
                    PopperAlign::Start => anchor.y + align_offset_val,
                    PopperAlign::Center => anchor.y + (anchor.height - content_height) / 2.0 + align_offset_val,
                    PopperAlign::End => anchor.y + anchor.height - content_height + align_offset_val,
                };
                (x, y)
            }
            PopperSide::Right => {
                let x = anchor.x + anchor.width + total_offset;
                let y = match align_val {
                    PopperAlign::Start => anchor.y + align_offset_val,
                    PopperAlign::Center => anchor.y + (anchor.height - content_height) / 2.0 + align_offset_val,
                    PopperAlign::End => anchor.y + anchor.height - content_height + align_offset_val,
                };
                (x, y)
            }
        };

        // Calculate available space for the popper based on placement side.
        // This measures how much room the popper has in the direction it's placed,
        // relative to the visual viewport bounds (for proper behavior during pinch-zoom).
        let (available_width, available_height) = match actual_side {
            PopperSide::Top => {
                // Popper above anchor: available height is space above anchor to visual viewport top
                (viewport_width, anchor.y - vv_top)
            }
            PopperSide::Bottom => {
                // Popper below anchor: available height is space below anchor to visual viewport bottom
                (viewport_width, vv_bottom - anchor.y - anchor.height)
            }
            PopperSide::Left => {
                // Popper left of anchor: available width is space to the left to visual viewport left
                (anchor.x - vv_left, viewport_height)
            }
            PopperSide::Right => {
                // Popper right of anchor: available width is space to the right to visual viewport right
                (vv_right - anchor.x - anchor.width, viewport_height)
            }
        };

        // Calculate arrow position (centered on anchor, relative to content)
        let arrow_position = if actual_side.is_horizontal() {
            // For left/right, arrow is positioned vertically
            let anchor_center_y = anchor.y + anchor.height / 2.0;
            let content_top = y;
            let pos = anchor_center_y - content_top - arrow_w / 2.0;
            // Clamp to content bounds with padding
            pos.max(8.0).min(content_height - arrow_w - 8.0)
        } else {
            // For top/bottom, arrow is positioned horizontally
            let anchor_center_x = anchor.x + anchor.width / 2.0;
            let content_left = x;
            let pos = anchor_center_x - content_left - arrow_w / 2.0;
            // Clamp to content bounds with padding
            pos.max(8.0).min(content_width - arrow_w - 8.0)
        };

        // Calculate transform origin (for animations)
        let (origin_x, origin_y) = match actual_side {
            PopperSide::Top => (arrow_position + arrow_w / 2.0, content_height),
            PopperSide::Bottom => (arrow_position + arrow_w / 2.0, 0.0),
            PopperSide::Left => (content_width, arrow_position + arrow_w / 2.0),
            PopperSide::Right => (0.0, arrow_position + arrow_w / 2.0),
        };

        // Update context signals
        placed_side.set(actual_side);
        placed_align.set(align_val);
        arrow_pos.set(arrow_position);
        should_hide_arrow.set(arrow_position < 0.0 || {
            if actual_side.is_horizontal() {
                arrow_position > content_height - arrow_w
            } else {
                arrow_position > content_width - arrow_w
            }
        });

        Some(PositionData {
            x,
            y,
            anchor_width: anchor.width,
            anchor_height: anchor.height,
            content_width,
            content_height,
            available_width,
            available_height,
            placed_side: actual_side,
            placed_align: align_val,
            arrow_pos: arrow_position,
            origin_x,
            origin_y,
        })
    };

    // Generate wrapper style
    let wrapper_style = move || {
        let Some(data) = position_data() else {
            return "position: fixed; left: 0px; top: 0px; visibility: hidden;".to_string();
        };

        format!(
            "position: fixed; left: 0px; top: 0px; transform: translate({:.1}px, {:.1}px); \
             will-change: transform; z-index: auto; \
             --radix-popper-available-width: {:.1}px; \
             --radix-popper-available-height: {:.1}px; \
             --radix-popper-anchor-width: {:.0}px; \
             --radix-popper-anchor-height: {:.0}px; \
             --radix-popper-transform-origin: {:.3}px {:.0}px;",
            data.x,
            data.y,
            data.available_width,
            data.available_height,
            data.anchor_width,
            data.anchor_height,
            data.origin_x,
            data.origin_y,
        )
    };

    let combined_style = move || {
        let base = wrapper_style();
        match &style {
            Some(s) => format!("{} {}", base, s),
            None => base,
        }
    };

    let data_side = move || placed_side.get().as_str();
    let data_align = move || placed_align.get().as_str();

    view! {
        <div
            node_ref=wrapper_ref
            data-radix-popper-content-wrapper=""
            style=combined_style
        >
            <div
                node_ref=content_ref
                class=class
                data-radix-popper-content=""
                data-side=data_side
                data-align=data_align
            >
                {children()}
            </div>
        </div>
    }
}

/// Arrow component that auto-positions based on placement.
#[component]
pub fn PopperArrow(
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
    let ctx =
        use_context::<PopperContentContext>().expect("PopperArrow must be used within PopperContent");

    // Set arrow dimensions in context
    ctx.arrow_width.set(width);
    ctx.arrow_height.set(height);

    // Arrow positioning based on placed side
    // Uses exact pixel positioning like React Radix
    let arrow_style = move || {
        let side = ctx.placed_side.get();
        let arrow_position = ctx.arrow_pos.get();
        let arrow_h = ctx.arrow_height.get() as f64;

        let visibility = if ctx.should_hide_arrow.get() {
            "visibility: hidden;"
        } else {
            ""
        };

        // Position and transform based on placed side
        // Arrow SVG points DOWN by default. Use pixel-based positioning like React Radix.
        let style = match side {
            PopperSide::Top => {
                // Arrow at bottom of content, pointing down toward anchor
                format!(
                    "position: absolute; left: {:.1}px; bottom: -{:.0}px; \
                     transform-origin: center 0px; {}",
                    arrow_position, arrow_h, visibility
                )
            }
            PopperSide::Bottom => {
                // Arrow at top of content, pointing up toward anchor
                // Position at top:0, translate up by arrow height, then rotate
                format!(
                    "position: absolute; left: {:.1}px; top: 0px; \
                     transform: rotate(180deg) translateY({:.0}px); \
                     transform-origin: center center; {}",
                    arrow_position, arrow_h, visibility
                )
            }
            PopperSide::Left => {
                // Arrow at right of content, pointing right toward anchor
                format!(
                    "position: absolute; top: {:.1}px; right: 0px; \
                     transform: translateY(50%) rotate(-90deg) translateX(50%); \
                     transform-origin: 100% 0px; {}",
                    arrow_position, visibility
                )
            }
            PopperSide::Right => {
                // Arrow at left of content, pointing left toward anchor
                format!(
                    "position: absolute; top: {:.1}px; left: 0px; \
                     transform: translateY(50%) rotate(90deg) translateX(-50%); \
                     transform-origin: 0% 0px; {}",
                    arrow_position, visibility
                )
            }
        };

        style
    };

    view! {
        <span
            data-radix-popper-arrow=""
            style=arrow_style
        >
            <crate::Arrow
                width=width
                height=height
                class=class.unwrap_or_default()
                style="display: block;"
            />
        </span>
    }
}
