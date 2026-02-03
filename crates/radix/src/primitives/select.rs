use leptos::portal::Portal;
use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// Helper to get visual viewport dimensions and offset.
fn get_visual_viewport() -> (f64, f64, f64, f64) {
    let Some(window) = web_sys::window() else {
        return (800.0, 600.0, 0.0, 0.0);
    };

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

/// Context shared between Select components.
#[derive(Clone, Copy)]
struct SelectContext {
    open: RwSignal<bool>,
    value: RwSignal<String>,
    on_value_change: Option<Callback<String>>,
    disabled: Signal<bool>,
    trigger_ref: NodeRef<leptos::html::Button>,
    /// Registry of item values to their display text
    item_texts: RwSignal<HashMap<String, String>>,
}

/// Context for SelectItem to communicate with SelectItemIndicator.
#[derive(Clone, Copy)]
struct SelectItemContext {
    is_selected: Signal<bool>,
}

/// Get all focusable select items within a content element.
fn get_select_items(content: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let node_list = content.query_selector_all("[data-radix-select-item]:not([data-disabled])");
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

/// Root container for a Select.
#[component]
pub fn SelectRoot(
    /// Controlled value.
    #[prop(optional, into)]
    value: Option<RwSignal<String>>,

    /// Default value (uncontrolled).
    #[prop(optional, into)]
    default_value: Option<String>,

    /// Callback when value changes.
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,

    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// Whether the select is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// The select parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(false));
    let value_signal = value.unwrap_or_else(|| RwSignal::new(default_value.unwrap_or_default()));
    let trigger_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let item_texts: RwSignal<HashMap<String, String>> = RwSignal::new(HashMap::new());

    // Notify open change callback
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    // Note: on_value_change is NOT called automatically when value changes.
    // It's only called when the user explicitly selects an item (in SelectItem/SelectContent).
    // This prevents infinite loops when the callback sets the same controlled signal.

    // Lock body scroll when open (must be in Root, not Content, so cleanup runs on close)
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
            let _ = style.set_property("overflow", "hidden");
        } else {
            let _ = style.remove_property("overflow");
        }
    });

    let ctx = SelectContext {
        open: open_signal,
        value: value_signal,
        on_value_change,
        disabled,
        trigger_ref,
        item_texts,
    };

    provide_context(ctx);

    children()
}

/// The trigger button that opens the select.
#[component]
pub fn SelectTrigger(
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
        use_context::<SelectContext>().expect("SelectTrigger must be used within SelectRoot");

    let open_signal = ctx.open;
    let disabled = ctx.disabled;
    let trigger_ref = ctx.trigger_ref;

    let on_click = move |_: web_sys::MouseEvent| {
        if !disabled.get() {
            open_signal.set(!open_signal.get_untracked());
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        match ev.key().as_str() {
            "Enter" | " " | "ArrowDown" | "ArrowUp" => {
                ev.prevent_default();
                open_signal.set(true);
            }
            _ => {}
        }
    };

    let data_state = move || if open_signal.get() { "open" } else { "closed" };
    let data_disabled = move || if disabled.get() { Some("") } else { None };

    view! {
        <button
            node_ref=trigger_ref
            type="button"
            role="combobox"
            aria-expanded=move || open_signal.get().to_string()
            aria-haspopup="listbox"
            aria-disabled=move || disabled.get().to_string()
            class=class
            style=style
            data-radix-select-trigger=""
            data-state=data_state
            data-disabled=data_disabled
            disabled=move || disabled.get()
            on:click=on_click
            on:keydown=on_keydown
        >
            {children()}
        </button>
    }
}

/// Displays the selected value or placeholder.
#[component]
pub fn SelectValue(
    /// Placeholder text when no value is selected.
    #[prop(optional, into)]
    placeholder: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectValue must be used within SelectRoot");

    let placeholder = StoredValue::new(placeholder);

    let display_text = move || {
        let value = ctx.value.get();
        if value.is_empty() {
            placeholder.get_value().unwrap_or_default()
        } else {
            // Look up the display text for this value
            ctx.item_texts
                .get()
                .get(&value)
                .cloned()
                .unwrap_or(value)
        }
    };

    let is_placeholder = move || ctx.value.get().is_empty();

    view! {
        <span
            class=class
            style=style
            data-radix-select-value=""
            data-placeholder=move || if is_placeholder() { Some("") } else { None }
        >
            {display_text}
        </span>
    }
}

/// Icon displayed in the trigger (typically a chevron).
#[component]
pub fn SelectIcon(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The icon content.
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class=class
            style=style
            data-radix-select-icon=""
            aria-hidden="true"
        >
            {children()}
        </span>
    }
}

/// Portal container - renders select content to document.body when open.
#[component]
pub fn SelectPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectPortal must be used within SelectRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || ctx.open.get()>
            <Portal>
                <SelectPortalInner ctx=ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn SelectPortalInner(ctx: SelectContext, children: StoredValue<ChildrenFn>) -> impl IntoView {
    provide_context(ctx);
    children.with_value(|c| c())
}

/// The dropdown content container.
#[component]
pub fn SelectContent(
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

    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<SelectContext>().expect("SelectContent must be used within SelectRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let trigger_ref = ctx.trigger_ref;

    // Position signal
    let position = RwSignal::new((0i32, 0i32, 0i32)); // x, y, width

    // Position content below trigger with collision detection
    Effect::new(move || {
        if open_signal.get() {
            if let Some(trigger_el) = trigger_ref.get() {
                let trigger_rect = trigger_el.get_bounding_client_rect();

                // Initial position below trigger, matching trigger width
                let initial_x = trigger_rect.left() as i32;
                let initial_y = trigger_rect.bottom() as i32 + 4; // 4px gap
                let width = trigger_rect.width() as i32;

                position.set((initial_x, initial_y, width));

                // Adjust after measuring content
                if let Some(window) = web_sys::window() {
                    let content_ref_inner = content_ref;
                    let trigger_top = trigger_rect.top() as i32;
                    let trigger_height = trigger_rect.height() as i32;
                    let callback = Closure::<dyn Fn()>::new(move || {
                        // Use try_get_untracked to handle case where component is disposed
                        let Some(Some(content_el)) = content_ref_inner.try_get_untracked() else {
                            return;
                        };
                        let content_rect = content_el.get_bounding_client_rect();
                        let content_height = content_rect.height() as i32;

                        let (vv_width, vv_height, vv_offset_x, vv_offset_y) =
                            get_visual_viewport();

                        let vv_left = vv_offset_x as i32 + collision_padding;
                        let vv_top = vv_offset_y as i32 + collision_padding;
                        let vv_right = vv_offset_x as i32 + vv_width as i32 - collision_padding;
                        let vv_bottom =
                            vv_offset_y as i32 + vv_height as i32 - collision_padding;

                        let mut x = initial_x;
                        let mut y = initial_y;

                        // Flip to above trigger if not enough space below
                        if y + content_height > vv_bottom {
                            let space_above = trigger_top - vv_top;
                            let space_below = vv_bottom - initial_y;
                            if space_above > space_below {
                                y = trigger_top - content_height - 4;
                            }
                        }

                        // Constrain to viewport bounds
                        if x + width > vv_right {
                            x = vv_right - width;
                        }
                        x = x.max(vv_left);
                        y = y.max(vv_top).min(vv_bottom - content_height);

                        position.set((x, y, width));

                        // Focus first selected item or first item
                        let html_el: web_sys::HtmlElement = content_el.clone().into();
                        let items = get_select_items(&html_el);

                        // Try to find and focus the selected item (use try to handle disposed)
                        let current_value = ctx.value.try_get_untracked().unwrap_or_default();
                        let selected_item = items.iter().find(|item| {
                            item.get_attribute("data-value")
                                .map(|v| v == current_value)
                                .unwrap_or(false)
                        });

                        if let Some(item) = selected_item {
                            let _ = item.focus();
                        } else if let Some(first) = items.first() {
                            let _ = first.focus();
                        }
                    });
                    let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
                    callback.forget();
                }
            }
        }
    });

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

                // Check if click is inside content (use try_get_untracked to handle disposed)
                if let Some(Some(content_el)) = content_ref.try_get_untracked() {
                    let content_node: &web_sys::Node = &content_el;
                    if content_node.contains(Some(target_node)) {
                        return;
                    }
                }

                // Check if click is inside trigger (use try_get_untracked to handle disposed)
                if let Some(Some(trigger_el)) = trigger_ref.try_get_untracked() {
                    let trigger_node: &web_sys::Node = &trigger_el;
                    if trigger_node.contains(Some(target_node)) {
                        return;
                    }
                }

                // Click was outside - close (check if signal still exists)
                let _ = open_signal.try_set(false);
            });

        let _ = document.add_event_listener_with_callback(
            "pointerdown",
            callback.as_ref().unchecked_ref(),
        );

        callback.forget();
    });

    // Handle keyboard navigation
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let Some(content_el) = content_ref.get() else {
            return;
        };
        let html_el: web_sys::HtmlElement = content_el.into();
        let items = get_select_items(&html_el);
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
                let new_index = current_index
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(count.saturating_sub(1));
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
                // Return focus to trigger
                if let Some(trigger_el) = trigger_ref.get() {
                    let _ = trigger_el.focus();
                }
            }
            "Tab" => {
                ev.prevent_default();
            }
            "Enter" | " " => {
                ev.prevent_default();
                // Select the focused item
                if let Some(idx) = current_index {
                    if let Some(item) = items.get(idx) {
                        if let Some(value) = item.get_attribute("data-value") {
                            ctx.value.set(value.clone());
                            if let Some(callback) = ctx.on_value_change {
                                callback.run(value);
                            }
                            open_signal.set(false);
                            if let Some(trigger_el) = trigger_ref.get() {
                                let _ = trigger_el.focus();
                            }
                        }
                    }
                }
            }
            key if key.len() == 1 => {
                // Typeahead - find first item starting with this character
                let char = key.to_lowercase();
                for item in &items {
                    if let Some(text) = item.text_content() {
                        if text.trim().to_lowercase().starts_with(&char) {
                            let _ = item.focus();
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let data_state = move || if open_signal.get() { "open" } else { "closed" };
    let class_val = StoredValue::new(class.unwrap_or_default());
    let style_val = StoredValue::new(style.unwrap_or_default());

    let position_style = move || {
        let (x, y, width) = position.get();
        let user_style = style_val.get_value();
        let base = format!(
            "position: fixed; left: {}px; top: {}px; min-width: {}px;",
            x, y, width
        );
        if user_style.is_empty() {
            base
        } else {
            format!("{} {}", base, user_style)
        }
    };

    view! {
        <div
            node_ref=content_ref
            role="listbox"
            tabindex="-1"
            class=class_val.get_value()
            style=position_style
            data-radix-select-content=""
            data-state=data_state
            on:keydown=on_keydown
        >
            {children.with_value(|c| c())}
        </div>
    }
}

/// Scrollable viewport for select items.
#[component]
pub fn SelectViewport(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The viewport content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-select-viewport=""
        >
            {children()}
        </div>
    }
}

/// A selectable item in the select.
#[component]
pub fn SelectItem(
    /// The value of this item.
    #[prop(into)]
    value: String,

    /// Whether the item is disabled.
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The item content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<SelectContext>().expect("SelectItem must be used within SelectRoot");

    let item_value = StoredValue::new(value.clone());
    let is_selected = Signal::derive(move || ctx.value.get() == item_value.get_value());
    let is_highlighted = RwSignal::new(false);

    // Provide context for SelectItemIndicator
    let item_ctx = SelectItemContext { is_selected };
    provide_context(item_ctx);

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        let val = item_value.get_value();
        ctx.value.set(val.clone());
        if let Some(callback) = ctx.on_value_change {
            callback.run(val);
        }
        ctx.open.set(false);
        // Return focus to trigger
        if let Some(trigger_el) = ctx.trigger_ref.get() {
            let _ = trigger_el.focus();
        }
    };

    let on_focus = move |_: web_sys::FocusEvent| {
        is_highlighted.set(true);
    };

    let on_blur = move |_: web_sys::FocusEvent| {
        is_highlighted.set(false);
    };

    let tabindex = move || if disabled.get() { None } else { Some("-1") };
    let data_state = move || if is_selected.get() { "checked" } else { "unchecked" };
    let data_highlighted = move || if is_highlighted.get() { Some("") } else { None };
    let data_disabled = move || if disabled.get() { Some("") } else { None };

    let children = StoredValue::new(children);

    view! {
        <div
            role="option"
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || disabled.get().to_string()
            tabindex=tabindex
            class=class
            style=style
            data-radix-select-item=""
            data-value=item_value.get_value()
            data-state=data_state
            data-highlighted=data_highlighted
            data-disabled=data_disabled
            on:click=on_click
            on:focus=on_focus
            on:blur=on_blur
        >
            {children.with_value(|c| c())}
        </div>
    }
}

/// Text content of a select item (used for display in SelectValue).
#[component]
pub fn SelectItemText(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The text content.
    children: Children,
) -> impl IntoView {
    let ctx =
        use_context::<SelectContext>().expect("SelectItemText must be used within SelectRoot");

    // Get the item value from the parent SelectItem's data-value attribute
    // We need to register the text with the root context so SelectValue can display it
    let node_ref: NodeRef<leptos::html::Span> = NodeRef::new();

    Effect::new(move || {
        if let Some(el) = node_ref.get() {
            // Find parent SelectItem to get the value
            let html_el: &web_sys::HtmlElement = &el;
            if let Some(parent) = html_el.closest("[data-radix-select-item]").ok().flatten() {
                if let Some(value) = parent.get_attribute("data-value") {
                    if let Some(text) = el.text_content() {
                        ctx.item_texts.update(|map| {
                            map.insert(value, text.trim().to_string());
                        });
                    }
                }
            }
        }
    });

    view! {
        <span
            node_ref=node_ref
            class=class
            style=style
            data-radix-select-item-text=""
        >
            {children()}
        </span>
    }
}

/// Indicator shown when item is selected.
#[component]
pub fn SelectItemIndicator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The indicator content.
    children: ChildrenFn,
) -> impl IntoView {
    let item_ctx = use_context::<SelectItemContext>();

    let should_show = move || item_ctx.map(|ctx| ctx.is_selected.get()).unwrap_or(false);

    let children = StoredValue::new(children);
    let class = StoredValue::new(class);
    let style = StoredValue::new(style);

    view! {
        <Show when=should_show>
            <span
                class=class.get_value()
                style=style.get_value()
                data-radix-select-item-indicator=""
                aria-hidden="true"
            >
                {children.with_value(|c| c())}
            </span>
        </Show>
    }
}

/// Group of select items.
#[component]
pub fn SelectGroup(
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
            data-radix-select-group=""
        >
            {children()}
        </div>
    }
}

/// Label for a group of items.
#[component]
pub fn SelectLabel(
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
            data-radix-select-label=""
        >
            {children()}
        </div>
    }
}

/// Visual separator between items or groups.
#[component]
pub fn SelectSeparator(
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
            data-radix-select-separator=""
        />
    }
}

/// Scroll up button for long lists.
#[component]
pub fn SelectScrollUpButton(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The button content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-select-scroll-up-button=""
            aria-hidden="true"
        >
            {children()}
        </div>
    }
}

/// Scroll down button for long lists.
#[component]
pub fn SelectScrollDownButton(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The button content.
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-select-scroll-down-button=""
            aria-hidden="true"
        >
            {children()}
        </div>
    }
}
