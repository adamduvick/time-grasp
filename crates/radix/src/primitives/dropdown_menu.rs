use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::primitives::popper::{
    PopperAlign, PopperAnchor, PopperContent, PopperContext, PopperRoot, PopperSide,
};

/// Re-export popper types for convenience
pub use crate::primitives::popper::{PopperAlign as DropdownMenuAlign, PopperSide as DropdownMenuSide};

/// Context shared between DropdownMenu components.
#[derive(Clone, Copy)]
struct DropdownMenuContext {
    open: RwSignal<bool>,
    /// Trigger to close menu after selection
    close_on_select: bool,
    /// Trigger element to return focus on close
    trigger_ref: NodeRef<leptos::html::Button>,
}

/// Get all focusable menu items within a content element.
fn get_menu_items(content: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let node_list = content.query_selector_all("[data-radix-dropdown-menu-item]:not([data-disabled])");
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

/// Root container for a dropdown menu.
#[component]
pub fn DropdownMenuRoot(
    /// Controlled open state.
    #[prop(optional)]
    open: Option<RwSignal<bool>>,

    /// Default open state for uncontrolled mode.
    #[prop(default = false)]
    default_open: bool,

    /// Whether to close the menu when an item is selected.
    #[prop(default = true)]
    close_on_select: bool,

    /// Callback when open state changes.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,

    /// The dropdown menu parts.
    children: Children,
) -> impl IntoView {
    let open_signal = open.unwrap_or_else(|| RwSignal::new(default_open));
    let trigger_ref: NodeRef<leptos::html::Button> = NodeRef::new();

    // Notify callback on changes
    if let Some(callback) = on_open_change {
        Effect::new(move || {
            callback.run(open_signal.get());
        });
    }

    let ctx = DropdownMenuContext {
        open: open_signal,
        close_on_select,
        trigger_ref,
    };

    provide_context(ctx);

    view! {
        <PopperRoot>
            {children()}
        </PopperRoot>
    }
}

/// Trigger button that toggles the dropdown menu.
#[component]
pub fn DropdownMenuTrigger(
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
    let ctx = use_context::<DropdownMenuContext>()
        .expect("DropdownMenuTrigger must be used within DropdownMenuRoot");

    let open_signal = ctx.open;
    let trigger_ref = ctx.trigger_ref;

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        open_signal.update(|open| *open = !*open);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }
        match ev.key().as_str() {
            " " | "Enter" | "ArrowDown" => {
                ev.prevent_default();
                open_signal.set(true);
            }
            "ArrowUp" => {
                ev.prevent_default();
                open_signal.set(true);
            }
            _ => {}
        }
    };

    let state_attr = move || if open_signal.get() { "open" } else { "closed" };

    view! {
        <PopperAnchor class=class.unwrap_or_default() style=style.unwrap_or_default()>
            <button
                type="button"
                node_ref=trigger_ref
                class="trigger-button"
                disabled=move || disabled.get()
                aria-haspopup="menu"
                aria-expanded=move || open_signal.get().to_string()
                data-radix-dropdown-menu-trigger=""
                data-state=state_attr
                on:click=on_click
                on:keydown=on_keydown
            >
                {children()}
            </button>
        </PopperAnchor>
    }
}

/// Portal container - renders dropdown menu to document.body when open.
#[component]
pub fn DropdownMenuPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    let menu_ctx = use_context::<DropdownMenuContext>()
        .expect("DropdownMenuPortal must be used within DropdownMenuRoot");
    let popper_ctx = use_context::<PopperContext>()
        .expect("DropdownMenuPortal must be used within DropdownMenuRoot");

    let children = StoredValue::new(children);

    view! {
        <Show when=move || menu_ctx.open.get()>
            <Portal>
                <DropdownMenuPortalInner menu_ctx=menu_ctx popper_ctx=popper_ctx children=children />
            </Portal>
        </Show>
    }
}

/// Inner component that re-provides context inside the portal.
#[component]
fn DropdownMenuPortalInner(
    menu_ctx: DropdownMenuContext,
    popper_ctx: PopperContext,
    children: StoredValue<ChildrenFn>,
) -> impl IntoView {
    provide_context(menu_ctx);
    provide_context(popper_ctx);
    children.with_value(|c| c())
}

/// The dropdown menu content.
#[component]
pub fn DropdownMenuContent(
    /// Which side of the trigger to show the menu.
    #[prop(into, default = Signal::derive(|| PopperSide::Bottom))]
    side: Signal<PopperSide>,

    /// Offset from the trigger in pixels.
    #[prop(into, default = Signal::derive(|| 0i32))]
    side_offset: Signal<i32>,

    /// Alignment along the side.
    #[prop(into, default = Signal::derive(|| PopperAlign::Start))]
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

    /// The menu content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<DropdownMenuContext>()
        .expect("DropdownMenuContent must be used within DropdownMenuRoot");

    let content_ref: NodeRef<leptos::html::Div> = node_ref;
    let children = StoredValue::new(children);

    let open_signal = ctx.open;
    let trigger_ref = ctx.trigger_ref;

    // Focus first item when menu opens
    Effect::new(move || {
        if open_signal.get() {
            // Use requestAnimationFrame to ensure DOM is ready
            if let Some(window) = web_sys::window() {
                let content_ref_inner = content_ref;
                let callback = Closure::<dyn Fn()>::new(move || {
                    if let Some(content_el) = content_ref_inner.get() {
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
                // Return focus to trigger
                if let Some(trigger_el) = trigger_ref.get() {
                    let html_el: &web_sys::HtmlElement = &trigger_el;
                    let _ = html_el.focus();
                }
            }
            "Tab" => {
                // Prevent Tab from escaping the dropdown - keep focus trapped
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

                // Check if click is on trigger
                if let Some(trigger_el) = trigger_ref.get() {
                    let trigger_node: &web_sys::Node = &trigger_el;
                    if trigger_node.contains(Some(target_node)) {
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
                role="menu"
                tabindex="-1"
                class=class_val.get_value()
                data-radix-dropdown-menu-content=""
                data-state=state_attr
                on:keydown=on_keydown
            >
                {children.with_value(|c| c())}
            </div>
        </PopperContent>
    }
}

/// A menu item that can be selected.
#[component]
pub fn DropdownMenuItem(
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
    let ctx = use_context::<DropdownMenuContext>()
        .expect("DropdownMenuItem must be used within DropdownMenuRoot");

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

    // Use tabindex -1 for items (focus managed by content's keydown handler)
    let tabindex = move || if disabled.get() { None } else { Some("-1") };

    view! {
        <div
            role="menuitem"
            tabindex=tabindex
            class=class
            style=style
            data-radix-dropdown-menu-item=""
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
pub fn DropdownMenuSeparator(
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
            data-radix-dropdown-menu-separator=""
        />
    }
}

/// A non-interactive label for a group of items.
#[component]
pub fn DropdownMenuLabel(
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
            data-radix-dropdown-menu-label=""
        >
            {children()}
        </div>
    }
}

/// A group of menu items.
#[component]
pub fn DropdownMenuGroup(
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
            data-radix-dropdown-menu-group=""
        >
            {children()}
        </div>
    }
}
