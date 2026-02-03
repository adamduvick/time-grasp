use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// Helper to get visual viewport dimensions and offset.
/// Returns (width, height, offset_left, offset_top).
/// Falls back to layout viewport if visualViewport API is not available.
fn get_visual_viewport() -> (f64, f64, f64, f64) {
    let Some(window) = web_sys::window() else {
        return (800.0, 600.0, 0.0, 0.0);
    };

    // Try to get visualViewport
    if let Ok(vv) = js_sys::Reflect::get(&window, &JsValue::from_str("visualViewport")) {
        if !vv.is_undefined() && !vv.is_null() {
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

            if width > 0.0 && height > 0.0 {
                return (width, height, offset_left, offset_top);
            }
        }
    }

    // Fallback to layout viewport
    let width = window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    let height = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(600.0);

    (width, height, 0.0, 0.0)
}

/// Context shared between ContextMenu components.
#[derive(Clone, Copy)]
struct ContextMenuContext {
    open: RwSignal<bool>,
    /// Position where the menu should appear (pointer location)
    position: RwSignal<(i32, i32)>,
    /// Whether to close menu after selection
    close_on_select: bool,
}

/// Context for a submenu.
#[derive(Clone, Copy)]
struct ContextMenuSubContext {
    open: RwSignal<bool>,
    /// Reference to the trigger element for positioning
    trigger_ref: NodeRef<leptos::html::Div>,
}

/// Get all focusable menu items within a content element.
fn get_menu_items(content: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    // Select both regular items and sub-triggers, excluding disabled ones
    let node_list = content.query_selector_all(
        "[data-radix-context-menu-item]:not([data-disabled]), [data-radix-context-menu-sub-trigger]:not([data-disabled])"
    );
    let Ok(nodes) = node_list else {
        return vec![];
    };

    let mut items = vec![];
    for i in 0..nodes.length() {
        if let Some(node) = nodes.get(i) {
            if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                items.push(el);
            }
        }
    }
    items
}

/// Get the index of the currently focused item.
fn get_focused_index(items: &[web_sys::HtmlElement]) -> Option<usize> {
    let document = web_sys::window()?.document()?;
    let active = document.active_element()?;

    items.iter().position(|item| {
        let item_el: &web_sys::Element = item;
        item_el == &active
    })
}

/// Root container for a context menu.
#[component]
pub fn ContextMenuRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Whether to close the menu when an item is selected.
    #[prop(default = true)]
    close_on_select: bool,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// The context menu parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(false));
    let position = RwSignal::new((0i32, 0i32));

    // Notify callback on changes
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    // Lock/unlock body scroll when menu opens/closes
    Effect::new(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };
        let style = body.style();

        if open_signal.get() {
            // Lock scroll
            let _ = style.set_property("overflow", "hidden");
        } else {
            // Restore scroll
            let _ = style.remove_property("overflow");
        }
    });

    let ctx = ContextMenuContext {
        open: open_signal,
        position,
        close_on_select,
    };

    provide_context(ctx);

    children()
}

/// Trigger area where right-click activates the context menu.
#[component]
pub fn ContextMenuTrigger(
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
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuTrigger must be used within ContextMenuRoot");

    let open_signal = ctx.open;
    let position = ctx.position;

    let on_contextmenu = move |ev: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        ev.prevent_default();
        position.set((ev.client_x(), ev.client_y()));
        open_signal.set(true);
    };

    view! {
        <div
            class=class
            style=style
            data-radix-context-menu-trigger=""
            data-state=move || if open_signal.get() { "open" } else { "closed" }
            on:contextmenu=on_contextmenu
        >
            {children()}
        </div>
    }
}

/// Portal container - renders context menu to document.body when open.
#[component]
pub fn ContextMenuPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuPortal must be used within ContextMenuRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <Portal>
                <ContextMenuPortalInner ctx=ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn ContextMenuPortalInner(
    ctx: ContextMenuContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(ctx);
    children.with_value(|c| c())
}

/// The context menu content.
#[component]
pub fn ContextMenuContent(
    /// Padding from viewport edges for collision detection.
    #[prop(default = 8)]
    collision_padding: i32,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The menu content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuContent must be used within ContextMenuRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let position = ctx.position;

    // Adjusted position after collision detection
    let adjusted_position = RwSignal::new((0i32, 0i32));

    // Collision detection and focus first item when menu opens
    Effect::new(move || {
        if open_signal.get() {
            if let Some(window) = web_sys::window() {
                let content_ref_inner = content_ref;
                let pos = position.get();
                let callback = Closure::<dyn Fn()>::new(move || {
                    if let Some(content_el) = content_ref_inner.get() {
                        let html_el: web_sys::HtmlElement = content_el.clone().into();

                        // Get menu dimensions
                        let rect = html_el.get_bounding_client_rect();
                        let menu_width = rect.width() as i32;
                        let menu_height = rect.height() as i32;

                        // Get visual viewport dimensions and offset for pinch-zoom support.
                        // The visual viewport may be smaller and offset from the layout viewport.
                        let (vv_width, vv_height, vv_offset_x, vv_offset_y) = get_visual_viewport();

                        // Calculate visual viewport bounds in layout viewport coordinates
                        let vv_left = vv_offset_x as i32 + collision_padding;
                        let vv_top = vv_offset_y as i32 + collision_padding;
                        let vv_right = vv_offset_x as i32 + vv_width as i32 - collision_padding;
                        let vv_bottom = vv_offset_y as i32 + vv_height as i32 - collision_padding;

                        let (mut x, mut y) = pos;

                        // Adjust X if menu would overflow right edge of visual viewport
                        if x + menu_width > vv_right {
                            x = (vv_right - menu_width).max(vv_left);
                        }

                        // Adjust Y if menu would overflow bottom edge of visual viewport
                        if y + menu_height > vv_bottom {
                            y = (vv_bottom - menu_height).max(vv_top);
                        }

                        // Ensure menu doesn't go off left/top edges of visual viewport
                        x = x.max(vv_left);
                        y = y.max(vv_top);

                        adjusted_position.set((x, y));

                        // Focus first item
                        let items = get_menu_items(&html_el);
                        if let Some(first_item) = items.first() {
                            let _ = first_item.focus();
                        }
                    }
                });
                let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                callback.forget();
            }
        }
    });

    // Initialize adjusted position from original position
    Effect::new(move || {
        adjusted_position.set(position.get());
    });

    // Handle keyboard navigation
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let Some(content_el) = content_ref.get() else {
            return;
        };
        let html_el: web_sys::HtmlElement = content_el.into();
        let items = get_menu_items(&html_el);
        let count = items.len();

        if count == 0 {
            return;
        }

        let current_index = get_focused_index(&items);

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let new_index = current_index.map(|i| (i + 1).min(count - 1)).unwrap_or(0);
                if let Some(el) = items.get(new_index) {
                    let _ = el.focus();
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                let new_index = current_index.map(|i| i.saturating_sub(1)).unwrap_or(count.saturating_sub(1));
                if let Some(el) = items.get(new_index) {
                    let _ = el.focus();
                }
            }
            "Home" => {
                ev.prevent_default();
                if let Some(el) = items.first() {
                    let _ = el.focus();
                }
            }
            "End" => {
                ev.prevent_default();
                if let Some(el) = items.last() {
                    let _ = el.focus();
                }
            }
            "Escape" => {
                ev.prevent_default();
                open_signal.set(false);
            }
            "Tab" => {
                // Prevent Tab from escaping the context menu
                ev.prevent_default();
            }
            _ => {}
        }
    };

    // Handle click outside
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

        let callback =
            Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |ev: web_sys::PointerEvent| {
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

                // Click was outside - close menu
                open_signal.set(false);
            });

        let _ = document.add_event_listener_with_callback(
            "pointerdown",
            callback.as_ref().unchecked_ref(),
        );

        callback.forget();
    });

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };
    let class_val = StoredValue::new(class.unwrap_or_default());

    // Position the menu at the adjusted location, combining with user style
    let style_val = StoredValue::new(style.unwrap_or_default());
    let position_style = move || {
        let (x, y) = adjusted_position.get();
        let user_style = style_val.get_value();
        if user_style.is_empty() {
            format!("position: fixed; left: {}px; top: {}px;", x, y)
        } else {
            format!("position: fixed; left: {}px; top: {}px; {}", x, y, user_style)
        }
    };

    view! {
        <div
            node_ref=content_ref
            role="menu"
            tabindex="-1"
            class=class_val.get_value()
            style=position_style
            data-radix-context-menu-content=""
            data-state=state_attr
            on:keydown=on_keydown
        >
            {children.with_value(|c| c())}
        </div>
    }
}

/// A menu item that can be selected.
#[component]
pub fn ContextMenuItem(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether the item is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// Callback when item is selected.
    #[prop(optional)]
    on_select: Option<Callback<()>>,

    /// The item content.
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuItem must be used within ContextMenuRoot");

    let open_signal = ctx.open;
    let close_on_select = ctx.close_on_select;
    let is_highlighted = RwSignal::new(false);

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        if let Some(callback) = on_select {
            callback.run(());
        }
        if close_on_select {
            open_signal.set(false);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            if let Some(callback) = on_select {
                callback.run(());
            }
            if close_on_select {
                open_signal.set(false);
            }
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        is_highlighted.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        is_highlighted.set(false);
    };

    let tabindex = move || if disabled.get() { None } else { Some("-1") };

    view! {
        <div
            role="menuitem"
            tabindex=tabindex
            class=class
            style=style
            data-radix-context-menu-item=""
            data-highlighted=move || if is_highlighted.get() { Some("") } else { None }
            data-disabled=move || if disabled.get() { Some("") } else { None }
            aria-disabled=move || disabled.get().to_string()
            on:click=on_click
            on:focus=on_focus
            on:blur=on_blur
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// A visual separator between menu items.
#[component]
pub fn ContextMenuSeparator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            class=class
            style=style
            data-radix-context-menu-separator=""
        />
    }
}

/// A non-interactive label for a group of items.
#[component]
pub fn ContextMenuLabel(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The label content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-context-menu-label=""
        >
            {children()}
        </div>
    }
}

/// A group of menu items.
#[component]
pub fn ContextMenuGroup(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The group content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            class=class
            style=style
            data-radix-context-menu-group=""
        >
            {children()}
        </div>
    }
}

/// Container for a submenu.
#[component]
pub fn ContextMenuSub(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// The submenu parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(false));
    let trigger_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Notify callback on changes
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    let sub_ctx = ContextMenuSubContext {
        open: open_signal,
        trigger_ref,
    };

    provide_context(sub_ctx);

    children()
}

/// Trigger that opens a submenu.
#[component]
pub fn ContextMenuSubTrigger(
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
    let sub_ctx = use_context::<ContextMenuSubContext>()
        .expect("ContextMenuSubTrigger must be used within ContextMenuSub");

    let open_signal = sub_ctx.open;
    let trigger_ref = sub_ctx.trigger_ref;
    let is_highlighted = RwSignal::new(false);

    let on_pointer_enter = move |_: web_sys::PointerEvent| {
        if !disabled.get() {
            open_signal.set(true);
        }
    };

    let on_pointer_leave = move |ev: web_sys::PointerEvent| {
        // Check if pointer moved to the submenu content
        if let Some(related) = ev.related_target() {
            if let Some(el) = related.dyn_ref::<web_sys::Element>() {
                // If moving to submenu content, don't close
                if el.closest("[data-radix-context-menu-sub-content]").ok().flatten().is_some() {
                    return;
                }
            }
        }
        open_signal.set(false);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        match ev.key().as_str() {
            "ArrowRight" | "Enter" | " " => {
                ev.prevent_default();
                ev.stop_propagation();
                open_signal.set(true);
            }
            _ => {}
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        is_highlighted.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        is_highlighted.set(false);
    };

    let tabindex = move || if disabled.get() { None } else { Some("-1") };

    view! {
        <div
            node_ref=trigger_ref
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=move || open_signal.get().to_string()
            tabindex=tabindex
            class=class
            style=style
            data-radix-context-menu-sub-trigger=""
            data-state=move || if open_signal.get() { "open" } else { "closed" }
            data-highlighted=move || if is_highlighted.get() { Some("") } else { None }
            data-disabled=move || if disabled.get() { Some("") } else { None }
            aria-disabled=move || disabled.get().to_string()
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:focus=on_focus
            on:blur=on_blur
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// Content of a submenu.
#[component]
pub fn ContextMenuSubContent(
    /// Offset from the trigger in pixels.
    #[prop(default = 2)]
    side_offset: i32,

    /// Alignment offset in pixels.
    #[prop(default = -5)]
    align_offset: i32,

    /// Padding from viewport edges for collision detection.
    #[prop(default = 8)]
    collision_padding: i32,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The submenu content.
    children: ChildrenFn,
) -> impl IntoView {
    let menu_ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuSubContent must be used within ContextMenuRoot");
    let sub_ctx = use_context::<ContextMenuSubContext>()
        .expect("ContextMenuSubContent must be used within ContextMenuSub");

    let content_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let children = StoredValue::new(children);

    let open_signal = sub_ctx.open;
    let trigger_ref = sub_ctx.trigger_ref;
    let menu_open = menu_ctx.open;

    // Close submenu when parent menu closes
    Effect::new(move || {
        if !menu_open.get() {
            open_signal.set(false);
        }
    });

    // Position signal for the submenu
    let position = RwSignal::new((0i32, 0i32));

    // Calculate position based on trigger element
    Effect::new(move || {
        if open_signal.get() {
            if let Some(trigger_el) = trigger_ref.get() {
                let rect = trigger_el.get_bounding_client_rect();

                // Position to the right of the trigger (initial)
                let initial_x = rect.right() as i32 + side_offset;
                let initial_y = rect.top() as i32 + align_offset;

                // Set initial position
                position.set((initial_x, initial_y));

                // Adjust after measuring content in rAF
                if let Some(window) = web_sys::window() {
                    let content_ref_inner = content_ref;
                    let trigger_left = rect.left() as i32;
                    let callback = Closure::<dyn Fn()>::new(move || {
                        if let Some(content_el) = content_ref_inner.get() {
                            let content_rect = content_el.get_bounding_client_rect();
                            let menu_width = content_rect.width() as i32;
                            let menu_height = content_rect.height() as i32;

                            // Get visual viewport dimensions and offset for pinch-zoom support
                            let (vv_width, vv_height, vv_offset_x, vv_offset_y) = get_visual_viewport();

                            // Calculate visual viewport bounds in layout viewport coordinates
                            let vv_left = vv_offset_x as i32 + collision_padding;
                            let vv_top = vv_offset_y as i32 + collision_padding;
                            let vv_right = vv_offset_x as i32 + vv_width as i32 - collision_padding;
                            let vv_bottom = vv_offset_y as i32 + vv_height as i32 - collision_padding;

                            // Start with initial position
                            let mut x = initial_x;
                            let mut y = initial_y;

                            // Adjust X if menu would overflow right edge of visual viewport - flip to left
                            if x + menu_width > vv_right {
                                x = trigger_left - menu_width - side_offset;
                            }

                            // Adjust Y if menu would overflow bottom edge of visual viewport
                            if y + menu_height > vv_bottom {
                                y = (vv_bottom - menu_height).max(vv_top);
                            }

                            // Ensure menu doesn't go off left/top edges of visual viewport
                            x = x.max(vv_left);
                            y = y.max(vv_top);

                            position.set((x, y));

                            // Focus first item
                            let html_el: web_sys::HtmlElement = content_el.into();
                            let items = get_menu_items(&html_el);
                            if let Some(first_item) = items.first() {
                                let _ = first_item.focus();
                            }
                        }
                    });
                    let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                    callback.forget();
                }
            }
        }
    });

    // Handle pointer leave on content
    let on_pointer_leave = move |ev: web_sys::PointerEvent| {
        // Check if pointer moved back to the trigger
        if let Some(related) = ev.related_target() {
            if let Some(el) = related.dyn_ref::<web_sys::Element>() {
                // If moving to trigger or staying in submenu, don't close
                if el.closest("[data-radix-context-menu-sub-trigger]").ok().flatten().is_some() {
                    return;
                }
                if el.closest("[data-radix-context-menu-sub-content]").ok().flatten().is_some() {
                    return;
                }
            }
        }
        open_signal.set(false);
    };

    // Handle keyboard navigation
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let Some(content_el) = content_ref.get() else {
            return;
        };
        let html_el: web_sys::HtmlElement = content_el.into();
        let items = get_menu_items(&html_el);
        let count = items.len();

        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                ev.stop_propagation();
                let current_index = get_focused_index(&items);
                let new_index = current_index.map(|i| (i + 1).min(count - 1)).unwrap_or(0);
                if let Some(el) = items.get(new_index) {
                    let _ = el.focus();
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                ev.stop_propagation();
                let current_index = get_focused_index(&items);
                let new_index = current_index.map(|i| i.saturating_sub(1)).unwrap_or(count.saturating_sub(1));
                if let Some(el) = items.get(new_index) {
                    let _ = el.focus();
                }
            }
            "ArrowLeft" | "Escape" => {
                ev.prevent_default();
                ev.stop_propagation();
                open_signal.set(false);
                // Return focus to trigger
                if let Some(trigger_el) = trigger_ref.get() {
                    let html_el: &web_sys::HtmlElement = &trigger_el;
                    let _ = html_el.focus();
                }
            }
            "Home" => {
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(el) = items.first() {
                    let _ = el.focus();
                }
            }
            "End" => {
                ev.prevent_default();
                ev.stop_propagation();
                if let Some(el) = items.last() {
                    let _ = el.focus();
                }
            }
            "Tab" => {
                ev.prevent_default();
            }
            _ => {}
        }
    };

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };
    let class_val = StoredValue::new(class.unwrap_or_default());
    let style_val = StoredValue::new(style.unwrap_or_default());

    let position_style = move || {
        let (x, y) = position.get();
        let user_style = style_val.get_value();
        if user_style.is_empty() {
            format!("position: fixed; left: {}px; top: {}px;", x, y)
        } else {
            format!("position: fixed; left: {}px; top: {}px; {}", x, y, user_style)
        }
    };

    view! {
        <Show when=move || open_signal.get()>
            <Portal>
                <div
                    node_ref=content_ref
                    role="menu"
                    tabindex="-1"
                    class=class_val.get_value()
                    style=position_style
                    data-radix-context-menu-sub-content=""
                    data-state=state_attr
                    on:pointerleave=on_pointer_leave
                    on:keydown=on_keydown
                >
                    {children.with_value(|c| c())}
                </div>
            </Portal>
        </Show>
    }
}

/// Context for radio group.
#[derive(Clone, Copy)]
struct ContextMenuRadioGroupContext {
    value: Signal<String>,
    on_value_change: Option<Callback<String>>,
}

/// Context for item indicator to know if parent is checked.
#[derive(Clone, Copy)]
struct ContextMenuItemCheckedContext {
    checked: Signal<bool>,
}

/// A checkbox menu item that can be toggled.
#[component]
pub fn ContextMenuCheckboxItem(
    /// Whether the item is checked.
    #[prop(into)]
    checked: Signal<bool>,

    /// Callback when checked state changes.
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether the item is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// The item content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuCheckboxItem must be used within ContextMenuRoot");

    let open_signal = ctx.open;
    let close_on_select = ctx.close_on_select;
    let is_highlighted = RwSignal::new(false);

    // Provide checked state context for ContextMenuItemIndicator
    let checked_ctx = ContextMenuItemCheckedContext { checked };
    provide_context(checked_ctx);

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        let new_value = !checked.get();
        if let Some(callback) = on_checked_change {
            callback.run(new_value);
        }
        if close_on_select {
            open_signal.set(false);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            let new_value = !checked.get();
            if let Some(callback) = on_checked_change {
                callback.run(new_value);
            }
            if close_on_select {
                open_signal.set(false);
            }
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        is_highlighted.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        is_highlighted.set(false);
    };

    let tabindex = move || if disabled.get() { None } else { Some("-1") };
    let state_attr = move || if checked.get() { "checked" } else { "unchecked" };
    let children = StoredValue::new(children);

    view! {
        <div
            role="menuitemcheckbox"
            aria-checked=move || checked.get().to_string()
            tabindex=tabindex
            class=class
            style=style
            data-radix-context-menu-item=""
            data-radix-context-menu-checkbox-item=""
            data-state=state_attr
            data-highlighted=move || if is_highlighted.get() { Some("") } else { None }
            data-disabled=move || if disabled.get() { Some("") } else { None }
            aria-disabled=move || disabled.get().to_string()
            on:click=on_click
            on:focus=on_focus
            on:blur=on_blur
            on:keydown=on_keydown
        >
            {children.with_value(|c| c())}
        </div>
    }
}

/// Container for radio menu items.
#[component]
pub fn ContextMenuRadioGroup(
    /// The current selected value.
    #[prop(into)]
    value: Signal<String>,

    /// Callback when value changes.
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The radio items.
    children: Children,
) -> impl IntoView {
    let radio_ctx = ContextMenuRadioGroupContext {
        value,
        on_value_change,
    };

    provide_context(radio_ctx);

    view! {
        <div
            role="group"
            class=class
            style=style
            data-radix-context-menu-radio-group=""
        >
            {children()}
        </div>
    }
}

/// A radio menu item within a radio group.
#[component]
pub fn ContextMenuRadioItem(
    /// The value of this radio item.
    #[prop(into)]
    value: String,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether the item is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// The item content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ContextMenuContext>()
        .expect("ContextMenuRadioItem must be used within ContextMenuRoot");
    let radio_ctx = use_context::<ContextMenuRadioGroupContext>()
        .expect("ContextMenuRadioItem must be used within ContextMenuRadioGroup");

    let open_signal = ctx.open;
    let close_on_select = ctx.close_on_select;
    let is_highlighted = RwSignal::new(false);

    let item_value = StoredValue::new(value);
    let is_checked = Signal::derive(move || radio_ctx.value.get() == item_value.get_value());

    // Provide checked state context for ContextMenuItemIndicator
    let checked_ctx = ContextMenuItemCheckedContext { checked: is_checked };
    provide_context(checked_ctx);

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        if let Some(callback) = radio_ctx.on_value_change {
            callback.run(item_value.get_value());
        }
        if close_on_select {
            open_signal.set(false);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            if let Some(callback) = radio_ctx.on_value_change {
                callback.run(item_value.get_value());
            }
            if close_on_select {
                open_signal.set(false);
            }
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        is_highlighted.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        is_highlighted.set(false);
    };

    let tabindex = move || if disabled.get() { None } else { Some("-1") };
    let state_attr = move || if is_checked.get() { "checked" } else { "unchecked" };
    let children = StoredValue::new(children);

    view! {
        <div
            role="menuitemradio"
            aria-checked=move || is_checked.get().to_string()
            tabindex=tabindex
            class=class
            style=style
            data-radix-context-menu-item=""
            data-radix-context-menu-radio-item=""
            data-state=state_attr
            data-highlighted=move || if is_highlighted.get() { Some("") } else { None }
            data-disabled=move || if disabled.get() { Some("") } else { None }
            aria-disabled=move || disabled.get().to_string()
            on:click=on_click
            on:focus=on_focus
            on:blur=on_blur
            on:keydown=on_keydown
        >
            {children.with_value(|c| c())}
        </div>
    }
}

/// Indicator that renders when a checkbox/radio item is checked.
#[component]
pub fn ContextMenuItemIndicator(
    /// Force show the indicator (for uncontrolled usage).
    #[prop(into, default = Signal::derive(|| None))]
    force_mount: Signal<Option<bool>>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The indicator content (e.g., checkmark or bullet).
    children: ChildrenFn,
) -> impl IntoView {
    // Read checked state from parent checkbox/radio item context
    let checked_ctx = use_context::<ContextMenuItemCheckedContext>();

    let should_show = move || {
        // If force_mount is Some(true), always show
        if let Some(true) = force_mount.get() {
            return true;
        }
        // Otherwise, show based on checked state from context
        checked_ctx.map(|ctx| ctx.checked.get()).unwrap_or(false)
    };

    let children = StoredValue::new(children);
    let class = StoredValue::new(class);
    let style = StoredValue::new(style);

    view! {
        <Show when=should_show>
            <span
                class=class.get_value()
                style=style.get_value()
                data-radix-context-menu-item-indicator=""
                aria-hidden="true"
            >
                {children.with_value(|c| c())}
            </span>
        </Show>
    }
}
