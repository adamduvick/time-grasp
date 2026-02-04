use leptos::portal::Portal;
use leptos::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::{
    PopperAlign, PopperAnchor, PopperArrow, PopperContent, PopperContext, PopperRoot, PopperSide,
};

// Counter for generating unique menu item IDs
static MENU_ITEM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Direction for menu navigation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MenuDir {
    #[default]
    Ltr,
    Rtl,
}

/// Context shared across the entire menu tree
#[derive(Clone, Copy)]
pub struct MenuContext {
    /// Whether the menu is open
    pub open: RwSignal<bool>,
    /// Callback when open state changes
    pub on_open_change: Option<Callback<bool>>,
    /// Whether this is a modal menu (traps focus)
    pub modal: bool,
    /// Text direction
    pub dir: MenuDir,
}

/// Context for menu content (focus management)
#[derive(Clone, Copy)]
struct MenuContentContext {
    /// Currently focused item ID
    focused_id: RwSignal<Option<u32>>,
    /// List of registered item IDs (in order)
    item_ids: RwSignal<Vec<u32>>,
    /// Item labels for typeahead (keyed by ID)
    item_labels: RwSignal<Vec<(u32, String)>>,
    /// Typeahead search string
    search_string: RwSignal<String>,
    /// Whether using keyboard navigation (vs pointer)
    is_using_keyboard: RwSignal<bool>,
    /// Reference to the content element for focus management
    content_id: StoredValue<String>,
    /// Callback to close the menu
    on_close: Option<Callback<()>>,
}

/// Context for checkbox/radio items to communicate checked state to indicator
#[derive(Clone, Copy)]
struct MenuItemCheckedContext {
    checked: Signal<bool>,
}

/// Context for radio groups
#[derive(Clone, Copy)]
struct MenuRadioGroupContext {
    value: RwSignal<String>,
    on_value_change: Option<Callback<String>>,
}

/// Root component for a menu
#[component]
pub fn MenuRoot(
    /// Controlled open state
    #[prop(optional, into)]
    open: Option<RwSignal<bool>>,

    /// Default open state (uncontrolled)
    #[prop(default = false)]
    default_open: bool,

    /// Callback when open state changes
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// Whether this is a modal menu (traps focus)
    #[prop(default = true)]
    modal: bool,

    /// Text direction
    #[prop(default = MenuDir::Ltr)]
    dir: MenuDir,

    /// The menu parts
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));

    let ctx = MenuContext {
        open: open_signal,
        on_open_change,
        modal,
        dir,
    };

    provide_context(ctx);

    view! {
        <PopperRoot>
            {children()}
        </PopperRoot>
    }
}

/// Anchor element for positioning the menu
#[component]
pub fn MenuAnchor(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// The anchor content
    children: Children,
) -> impl IntoView {
    view! {
        <PopperAnchor class=class.unwrap_or_default() style=style.unwrap_or_default()>
            {children()}
        </PopperAnchor>
    }
}

/// Portal wrapper for menu content
#[component]
pub fn MenuPortal(
    /// The content to portal
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Portal>
            {children.with_value(|c| c())}
        </Portal>
    }
}

/// Menu content container with keyboard navigation
#[component]
pub fn MenuContent(
    /// Which side of the anchor to show the content
    #[prop(into, default = Signal::derive(|| PopperSide::Bottom))]
    side: Signal<PopperSide>,

    /// Offset from the anchor in pixels
    #[prop(into, default = Signal::derive(|| 0i32))]
    side_offset: Signal<i32>,

    /// Alignment along the side
    #[prop(into, default = Signal::derive(|| PopperAlign::Start))]
    align: Signal<PopperAlign>,

    /// Offset along the alignment axis
    #[prop(into, default = Signal::derive(|| 0i32))]
    align_offset: Signal<i32>,

    /// Whether to avoid collisions with viewport boundaries
    #[prop(into, default = Signal::derive(|| true))]
    avoid_collisions: Signal<bool>,

    /// Padding from viewport edges for collision detection
    #[prop(into, default = Signal::derive(|| 8i32))]
    collision_padding: Signal<i32>,

    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// Callback when escape key is pressed
    #[prop(optional)]
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,

    /// Callback when pointer moves outside
    #[prop(optional)]
    on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,

    /// Callback when focus moves outside
    #[prop(optional)]
    on_focus_outside: Option<Callback<web_sys::FocusEvent>>,

    /// The menu content
    children: ChildrenFn,
) -> impl IntoView {
    let menu_ctx = use_context::<MenuContext>().expect("MenuContent must be used within MenuRoot");

    let content_id = StoredValue::new(format!(
        "menu-content-{}",
        MENU_ITEM_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
    ));

    let focused_id = RwSignal::new(None::<u32>);
    let item_ids = RwSignal::new(Vec::<u32>::new());
    let item_labels: RwSignal<Vec<(u32, String)>> = RwSignal::new(Vec::new());
    let search_string = RwSignal::new(String::new());
    let is_using_keyboard = RwSignal::new(false);

    let on_close = menu_ctx.on_open_change.map(|cb| {
        Callback::new(move |_: ()| {
            cb.run(false);
        })
    });

    let content_ctx = MenuContentContext {
        focused_id,
        item_ids,
        item_labels,
        search_string,
        is_using_keyboard,
        content_id,
        on_close,
    };

    // Reset state when menu closes
    Effect::new(move || {
        if !menu_ctx.open.get() {
            focused_id.set(None);
            item_ids.set(Vec::new());
            item_labels.set(Vec::new());
            search_string.set(String::new());
            is_using_keyboard.set(false);
        }
    });

    // Close on outside click
    Effect::new(move || {
        if !menu_ctx.open.get() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        let content_id_val = content_id.get_value();
        let callback =
            Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |ev: web_sys::PointerEvent| {
                if let Some(target) = ev.target() {
                    if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                        // Check if click is inside menu content
                        let content_el = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.get_element_by_id(&content_id_val));

                        if let Some(content) = content_el {
                            if !content.contains(Some(&element)) {
                                if let Some(cb) = on_pointer_down_outside {
                                    cb.run(ev);
                                }
                                if let Some(is_open) = menu_ctx.open.try_get() {
                                    if is_open {
                                        menu_ctx.open.set(false);
                                        if let Some(cb) = menu_ctx.on_open_change {
                                            cb.run(false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });

        let _ =
            document.add_event_listener_with_callback("pointerdown", callback.as_ref().unchecked_ref());
        callback.forget();
    });

    let class = StoredValue::new(class);
    let style = StoredValue::new(style);
    let children = StoredValue::new(children);

    view! {
        <Show when=move || menu_ctx.open.get()>
            <MenuContentInner
                content_ctx=content_ctx
                content_id=content_id.get_value()
                side=side
                side_offset=side_offset
                align=align
                align_offset=align_offset
                avoid_collisions=avoid_collisions
                collision_padding=collision_padding
                class=class.get_value()
                style=style.get_value()
                children=children
            />
        </Show>
    }
}

/// Inner component to provide context inside Portal scope
#[component]
fn MenuContentInner(
    content_ctx: MenuContentContext,
    content_id: String,
    #[prop(into)] side: Signal<PopperSide>,
    #[prop(into)] side_offset: Signal<i32>,
    #[prop(into)] align: Signal<PopperAlign>,
    #[prop(into)] align_offset: Signal<i32>,
    #[prop(into)] avoid_collisions: Signal<bool>,
    #[prop(into)] collision_padding: Signal<i32>,
    class: Option<String>,
    style: Option<String>,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(content_ctx);

    let content_id = StoredValue::new(content_id);
    let menu_ctx = use_context::<MenuContext>().expect("MenuContentInner requires MenuContext");
    let popper_ctx = use_context::<PopperContext>();

    // Focus the content on mount
    Effect::new(move || {
        let id = content_id.get_value();
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = document.get_element_by_id(&id) {
                if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            }
        }
    });

    // Typeahead timeout
    let search_timeout = RwSignal::new(-1i32);
    let search_string = content_ctx.search_string;
    let focused_id = content_ctx.focused_id;
    let item_ids = content_ctx.item_ids;
    let item_labels = content_ctx.item_labels;
    let is_using_keyboard = content_ctx.is_using_keyboard;

    let clear_search_timeout = move || {
        let id = search_timeout.get_untracked();
        if id >= 0 {
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(id);
            }
        }
        search_timeout.set(-1);
    };

    let start_search_timeout = move || {
        clear_search_timeout();
        if let Some(window) = web_sys::window() {
            let callback = Closure::<dyn Fn()>::new(move || {
                search_string.set(String::new());
            });
            if let Ok(id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                1000,
            ) {
                search_timeout.set(id);
            }
            callback.forget();
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        is_using_keyboard.set(true);
        let key = ev.key();
        let ids = item_ids.get();
        let current = focused_id.get();

        let current_idx = current.and_then(|id| ids.iter().position(|&i| i == id));

        match key.as_str() {
            "Tab" => {
                if menu_ctx.modal {
                    ev.prevent_default();
                }
            }
            "ArrowDown" => {
                ev.prevent_default();
                if !ids.is_empty() {
                    let next_idx = match current_idx {
                        Some(idx) => (idx + 1) % ids.len(),
                        None => 0,
                    };
                    focused_id.set(Some(ids[next_idx]));
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                if !ids.is_empty() {
                    let prev_idx = match current_idx {
                        Some(idx) => {
                            if idx == 0 { ids.len() - 1 } else { idx - 1 }
                        }
                        None => ids.len() - 1,
                    };
                    focused_id.set(Some(ids[prev_idx]));
                }
            }
            "Home" | "PageUp" => {
                ev.prevent_default();
                if !ids.is_empty() {
                    focused_id.set(Some(ids[0]));
                }
            }
            "End" | "PageDown" => {
                ev.prevent_default();
                if !ids.is_empty() {
                    focused_id.set(Some(ids[ids.len() - 1]));
                }
            }
            "Escape" => {
                ev.prevent_default();
                menu_ctx.open.set(false);
                if let Some(cb) = menu_ctx.on_open_change {
                    cb.run(false);
                }
                // Return focus to anchor
                if let Some(ctx) = popper_ctx {
                    if let Some(anchor_el) = ctx.anchor_ref.get() {
                        // Try to find a focusable child (button, input, etc.) or use the container itself
                        let focusable = anchor_el
                            .query_selector("button, [tabindex], input, a, select, textarea")
                            .ok()
                            .flatten()
                            .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                            .or_else(|| anchor_el.clone().dyn_into::<web_sys::HtmlElement>().ok());

                        if let Some(el) = focusable {
                            let _ = el.focus();
                        }
                    }
                }
            }
            _ => {
                if key.len() == 1 {
                    let ch = key.chars().next().unwrap();
                    if ch.is_alphanumeric() || ch == ' ' {
                        ev.prevent_default();
                        let mut current_search = search_string.get();
                        current_search.push(ch.to_ascii_lowercase());
                        search_string.set(current_search.clone());

                        let labels = item_labels.get();
                        let start_idx = current_idx.map(|i| i + 1).unwrap_or(0);

                        for i in 0..labels.len() {
                            let idx = (start_idx + i) % labels.len();
                            let (id, label) = &labels[idx];
                            if label.to_lowercase().starts_with(&current_search) {
                                focused_id.set(Some(*id));
                                break;
                            }
                        }

                        start_search_timeout();
                    }
                }
            }
        }
    };

    let on_pointermove = move |_: web_sys::PointerEvent| {
        is_using_keyboard.set(false);
    };

    view! {
        <PopperContent
            side=side
            side_offset=side_offset
            align=align
            align_offset=align_offset
            avoid_collisions=avoid_collisions
            collision_padding=collision_padding
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
        >
            <div
                id=content_id.get_value()
                role="menu"
                tabindex="-1"
                data-radix-menu-content=""
                on:keydown=on_keydown
                on:pointermove=on_pointermove
            >
                {children.with_value(|c| c())}
            </div>
        </PopperContent>
    }
}

/// Group of menu items
#[component]
pub fn MenuGroup(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// The group content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            class=class
            style=style
            data-radix-menu-group=""
        >
            {children()}
        </div>
    }
}

/// Label for a group of menu items
#[component]
pub fn MenuLabel(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// The label content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            style=style
            data-radix-menu-label=""
        >
            {children()}
        </div>
    }
}

/// Individual menu item
#[component]
pub fn MenuItem(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled
    #[prop(default = false)]
    disabled: bool,

    /// Text value for typeahead
    #[prop(optional, into)]
    text_value: Option<String>,

    /// Callback when item is selected
    #[prop(optional)]
    on_select: Option<Callback<()>>,

    /// The item content
    children: Children,
) -> impl IntoView {
    let menu_ctx = use_context::<MenuContext>().expect("MenuItem must be used within MenuRoot");
    let content_ctx =
        use_context::<MenuContentContext>().expect("MenuItem must be used within MenuContent");

    // Generate unique ID and register
    let item_id = MENU_ITEM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    // Register item
    if !disabled {
        content_ctx.item_ids.update(|ids| ids.push(item_id));
        if let Some(label) = text_value.clone() {
            content_ctx
                .item_labels
                .update(|labels| labels.push((item_id, label)));
        }
    }

    let is_focused = Signal::derive(move || content_ctx.focused_id.get() == Some(item_id));

    let node_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    // Focus element when it becomes focused
    Effect::new(move || {
        if is_focused.get() && content_ctx.is_using_keyboard.get() {
            if let Some(Some(el)) = node_ref.try_get() {
                let html_el: web_sys::HtmlElement = el.into();
                let _ = html_el.focus();
            }
        }
    });

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled {
            return;
        }
        if let Some(cb) = on_select {
            cb.run(());
        }
        // Close menu after selection
        menu_ctx.open.set(false);
        if let Some(cb) = menu_ctx.on_open_change {
            cb.run(false);
        }
    };

    let on_pointerenter = move |_: web_sys::PointerEvent| {
        if !disabled {
            content_ctx.focused_id.set(Some(item_id));
        }
    };

    let on_pointerleave = move |_: web_sys::PointerEvent| {
        if content_ctx.focused_id.get() == Some(item_id) {
            content_ctx.focused_id.set(None);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled {
            return;
        }
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            if let Some(cb) = on_select {
                cb.run(());
            }
            menu_ctx.open.set(false);
            if let Some(cb) = menu_ctx.on_open_change {
                cb.run(false);
            }
        }
    };

    view! {
        <div
            node_ref=node_ref
            role="menuitem"
            class=class
            style=style
            tabindex="-1"
            data-radix-menu-item=""
            data-disabled=disabled.then_some("")
            data-highlighted=move || is_focused.get().then_some("")
            aria-disabled=disabled.then_some("true")
            on:click=on_click
            on:pointerenter=on_pointerenter
            on:pointerleave=on_pointerleave
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// Checkbox menu item
#[component]
pub fn MenuCheckboxItem(
    /// Checked state
    #[prop(into)]
    checked: RwSignal<bool>,

    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled
    #[prop(default = false)]
    disabled: bool,

    /// Text value for typeahead
    #[prop(optional, into)]
    text_value: Option<String>,

    /// Callback when checked state changes
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,

    /// The item content
    children: Children,
) -> impl IntoView {
    let content_ctx =
        use_context::<MenuContentContext>().expect("MenuCheckboxItem must be used within MenuContent");

    // Generate unique ID and register
    let item_id = MENU_ITEM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    if !disabled {
        content_ctx.item_ids.update(|ids| ids.push(item_id));
        if let Some(label) = text_value.clone() {
            content_ctx
                .item_labels
                .update(|labels| labels.push((item_id, label)));
        }
    }

    let is_focused = Signal::derive(move || content_ctx.focused_id.get() == Some(item_id));

    // Provide checked context for indicator
    let checked_ctx = MenuItemCheckedContext {
        checked: checked.into(),
    };
    provide_context(checked_ctx);

    let node_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    Effect::new(move || {
        if is_focused.get() && content_ctx.is_using_keyboard.get() {
            if let Some(Some(el)) = node_ref.try_get() {
                let html_el: web_sys::HtmlElement = el.into();
                let _ = html_el.focus();
            }
        }
    });

    let toggle = move || {
        if disabled {
            return;
        }
        let new_value = !checked.get();
        checked.set(new_value);
        if let Some(cb) = on_checked_change {
            cb.run(new_value);
        }
    };

    let on_click = move |_: web_sys::MouseEvent| {
        toggle();
    };

    let on_pointerenter = move |_: web_sys::PointerEvent| {
        if !disabled {
            content_ctx.focused_id.set(Some(item_id));
        }
    };

    let on_pointerleave = move |_: web_sys::PointerEvent| {
        if content_ctx.focused_id.get() == Some(item_id) {
            content_ctx.focused_id.set(None);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            toggle();
        }
    };

    let data_state = move || if checked.get() { "checked" } else { "unchecked" };

    view! {
        <div
            node_ref=node_ref
            role="menuitemcheckbox"
            class=class
            style=style
            tabindex="-1"
            data-radix-menu-checkbox-item=""
            data-state=data_state
            data-disabled=disabled.then_some("")
            data-highlighted=move || is_focused.get().then_some("")
            aria-checked=move || checked.get().to_string()
            aria-disabled=disabled.then_some("true")
            on:click=on_click
            on:pointerenter=on_pointerenter
            on:pointerleave=on_pointerleave
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// Radio group within menu
#[component]
pub fn MenuRadioGroup(
    /// Current value
    #[prop(into)]
    value: RwSignal<String>,

    /// Callback when value changes
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,

    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// The group content
    children: Children,
) -> impl IntoView {
    let ctx = MenuRadioGroupContext {
        value,
        on_value_change,
    };
    provide_context(ctx);

    view! {
        <div
            role="group"
            class=class
            style=style
            data-radix-menu-radio-group=""
        >
            {children()}
        </div>
    }
}

/// Radio item within menu
#[component]
pub fn MenuRadioItem(
    /// Value for this radio item
    #[prop(into)]
    value: String,

    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled
    #[prop(default = false)]
    disabled: bool,

    /// Text value for typeahead
    #[prop(optional, into)]
    text_value: Option<String>,

    /// The item content
    children: Children,
) -> impl IntoView {
    let content_ctx =
        use_context::<MenuContentContext>().expect("MenuRadioItem must be used within MenuContent");
    let radio_ctx = use_context::<MenuRadioGroupContext>()
        .expect("MenuRadioItem must be used within MenuRadioGroup");

    let item_value = StoredValue::new(value);

    // Generate unique ID and register
    let item_id = MENU_ITEM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    if !disabled {
        content_ctx.item_ids.update(|ids| ids.push(item_id));
        if let Some(label) = text_value.clone() {
            content_ctx
                .item_labels
                .update(|labels| labels.push((item_id, label)));
        }
    }

    let is_focused = Signal::derive(move || content_ctx.focused_id.get() == Some(item_id));
    let is_checked = Signal::derive(move || radio_ctx.value.get() == item_value.get_value());

    // Provide checked context for indicator
    let checked_ctx = MenuItemCheckedContext { checked: is_checked };
    provide_context(checked_ctx);

    let node_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    Effect::new(move || {
        if is_focused.get() && content_ctx.is_using_keyboard.get() {
            if let Some(Some(el)) = node_ref.try_get() {
                let html_el: web_sys::HtmlElement = el.into();
                let _ = html_el.focus();
            }
        }
    });

    let select = move || {
        if disabled {
            return;
        }
        let val = item_value.get_value();
        radio_ctx.value.set(val.clone());
        if let Some(cb) = radio_ctx.on_value_change {
            cb.run(val);
        }
    };

    let on_click = move |_: web_sys::MouseEvent| {
        select();
    };

    let on_pointerenter = move |_: web_sys::PointerEvent| {
        if !disabled {
            content_ctx.focused_id.set(Some(item_id));
        }
    };

    let on_pointerleave = move |_: web_sys::PointerEvent| {
        if content_ctx.focused_id.get() == Some(item_id) {
            content_ctx.focused_id.set(None);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" || ev.key() == " " {
            ev.prevent_default();
            select();
        }
    };

    let data_state = move || if is_checked.get() { "checked" } else { "unchecked" };

    view! {
        <div
            node_ref=node_ref
            role="menuitemradio"
            class=class
            style=style
            tabindex="-1"
            data-radix-menu-radio-item=""
            data-state=data_state
            data-disabled=disabled.then_some("")
            data-highlighted=move || is_focused.get().then_some("")
            aria-checked=move || is_checked.get().to_string()
            aria-disabled=disabled.then_some("true")
            on:click=on_click
            on:pointerenter=on_pointerenter
            on:pointerleave=on_pointerleave
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// Indicator for checkbox/radio items (renders children only when checked)
#[component]
pub fn MenuItemIndicator(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,

    /// Force mount even when unchecked
    #[prop(default = false)]
    force_mount: bool,

    /// The indicator content
    children: ChildrenFn,
) -> impl IntoView {
    let checked_ctx = use_context::<MenuItemCheckedContext>();

    let is_checked = move || checked_ctx.map(|ctx| ctx.checked.get()).unwrap_or(false);
    let children = StoredValue::new(children);

    view! {
        <Show when=move || force_mount || is_checked()>
            <span
                class=class.clone()
                style=style.clone()
                data-radix-menu-item-indicator=""
                data-state=move || if is_checked() { "checked" } else { "unchecked" }
            >
                {children.with_value(|c| c())}
            </span>
        </Show>
    }
}

/// Separator between menu items
#[component]
pub fn MenuSeparator(
    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            class=class
            style=style
            data-radix-menu-separator=""
            aria-orientation="horizontal"
        />
    }
}

/// Arrow pointing to the anchor
#[component]
pub fn MenuArrow(
    /// Width of the arrow in pixels
    #[prop(default = 10)]
    width: u32,

    /// Height of the arrow in pixels
    #[prop(default = 5)]
    height: u32,

    /// CSS class name(s) for styling
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    view! {
        <PopperArrow width=width height=height class=class.unwrap_or_default() />
    }
}
