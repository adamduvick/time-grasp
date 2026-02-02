use leptos::html::Button;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

/// Orientation of the toolbar for keyboard navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolbarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Toggle group mode within toolbar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolbarToggleType {
    #[default]
    Single,
    Multiple,
}

static TOOLBAR_ITEM_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Context shared between Toolbar components.
#[derive(Clone, Copy)]
struct ToolbarContext {
    orientation: ToolbarOrientation,
    /// The ID of the currently focused/tabbable item
    current_item_id: RwSignal<Option<usize>>,
    /// The ID of the first registered item (for initial tabindex)
    first_item_id: RwSignal<Option<usize>>,
}

/// Context for ToolbarToggleGroup.
#[derive(Clone)]
struct ToolbarToggleGroupContext {
    toggle_type: ToolbarToggleType,
    value: RwSignal<Vec<String>>,
}

impl ToolbarToggleGroupContext {
    fn is_pressed(&self, item_value: &str) -> bool {
        self.value.get().contains(&item_value.to_string())
    }

    fn toggle(&self, item_value: &str) {
        match self.toggle_type {
            ToolbarToggleType::Single => {
                self.value.update(|v| {
                    if v.contains(&item_value.to_string()) {
                        v.clear();
                    } else {
                        v.clear();
                        v.push(item_value.to_string());
                    }
                });
            }
            ToolbarToggleType::Multiple => {
                self.value.update(|v| {
                    if let Some(pos) = v.iter().position(|x| x == item_value) {
                        v.remove(pos);
                    } else {
                        v.push(item_value.to_string());
                    }
                });
            }
        }
    }
}

/// Register an item with the toolbar and return whether it should be initially tabbable.
fn register_toolbar_item(ctx: ToolbarContext, item_id: usize, disabled: bool) {
    // If this is the first non-disabled item, set it as the first item
    if !disabled {
        ctx.first_item_id.update(|first| {
            if first.is_none() {
                *first = Some(item_id);
            }
        });
    }
}

/// Get the tabindex for a toolbar item.
fn get_tabindex(ctx: ToolbarContext, item_id: usize) -> &'static str {
    let current = ctx.current_item_id.get();
    let first = ctx.first_item_id.get();

    match current {
        // If an item has been focused, only that item is tabbable
        Some(id) if id == item_id => "0",
        Some(_) => "-1",
        // If no item has been focused yet, the first registered item is tabbable
        None => {
            if first == Some(item_id) {
                "0"
            } else {
                "-1"
            }
        }
    }
}

/// Navigate focus within toolbar using arrow keys.
/// Returns the ID of the newly focused item if navigation occurred.
fn navigate_toolbar(
    current: web_sys::HtmlElement,
    direction: i32,
) -> Option<usize> {
    // Find the toolbar root
    let mut element = current.parent_element();
    let mut root: Option<web_sys::Element> = None;
    while let Some(el) = element {
        if el.has_attribute("data-radix-toolbar-root") {
            root = Some(el);
            break;
        }
        element = el.parent_element();
    }

    let root_el = root?;

    // Find all focusable items in the toolbar
    let items = root_el
        .query_selector_all("[data-radix-toolbar-item]:not([disabled])")
        .ok()?;

    let count = items.length() as i32;
    if count == 0 {
        return None;
    }

    // Find current index
    let mut current_index = 0i32;
    for i in 0..count {
        if let Some(el) = items.get(i as u32) {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                if html_el == current {
                    current_index = i;
                    break;
                }
            }
        }
    }

    // Calculate target index based on direction
    let target_index = if direction == 0 {
        // Home - find first enabled
        (0..count).find(|&i| {
            items
                .get(i as u32)
                .and_then(|node| node.dyn_ref::<web_sys::Element>().map(|el| !is_disabled(el)))
                .unwrap_or(false)
        })
    } else if direction == i32::MAX {
        // End - find last enabled
        (0..count).rev().find(|&i| {
            items
                .get(i as u32)
                .and_then(|node| node.dyn_ref::<web_sys::Element>().map(|el| !is_disabled(el)))
                .unwrap_or(false)
        })
    } else {
        // Arrow navigation - find next enabled, wrapping
        (1..=count).find_map(|offset| {
            let i = (current_index + direction * offset).rem_euclid(count);
            let is_enabled = items
                .get(i as u32)
                .and_then(|node| node.dyn_ref::<web_sys::Element>().map(|el| !is_disabled(el)))
                .unwrap_or(false);
            if is_enabled {
                Some(i)
            } else {
                None
            }
        })
    };

    let target_idx = target_index?;
    let target = items.get(target_idx as u32)?;
    let target_el = target.dyn_into::<web_sys::HtmlElement>().ok()?;

    // Get the item ID from the target element
    let item_id = target_el
        .get_attribute("data-radix-toolbar-item-id")
        .and_then(|id| id.parse::<usize>().ok());

    let _ = target_el.focus();

    item_id
}

fn is_disabled(el: &web_sys::Element) -> bool {
    if let Some(button) = el.dyn_ref::<web_sys::HtmlButtonElement>() {
        return button.disabled();
    }
    el.has_attribute("data-disabled")
}

fn handle_toolbar_keydown(
    ev: web_sys::KeyboardEvent,
    orientation: ToolbarOrientation,
    current_item_id: RwSignal<Option<usize>>,
) {
    let direction = match ev.key().as_str() {
        "ArrowRight" if orientation == ToolbarOrientation::Horizontal => Some(1i32),
        "ArrowLeft" if orientation == ToolbarOrientation::Horizontal => Some(-1i32),
        "ArrowDown" if orientation == ToolbarOrientation::Vertical => Some(1i32),
        "ArrowUp" if orientation == ToolbarOrientation::Vertical => Some(-1i32),
        // Also handle cross-axis arrows for convenience
        "ArrowDown" if orientation == ToolbarOrientation::Horizontal => Some(1i32),
        "ArrowUp" if orientation == ToolbarOrientation::Horizontal => Some(-1i32),
        "ArrowRight" if orientation == ToolbarOrientation::Vertical => Some(1i32),
        "ArrowLeft" if orientation == ToolbarOrientation::Vertical => Some(-1i32),
        "Home" => Some(0i32),
        "End" => Some(i32::MAX),
        _ => None,
    };

    let Some(dir) = direction else {
        return;
    };

    ev.prevent_default();

    let Some(current_target) = ev.current_target() else {
        return;
    };
    let Ok(current) = current_target.dyn_into::<web_sys::HtmlElement>() else {
        return;
    };

    if let Some(new_id) = navigate_toolbar(current, dir) {
        current_item_id.set(Some(new_id));
    }
}

/// Root container for a toolbar.
#[component]
pub fn ToolbarRoot(
    /// Orientation affects keyboard navigation.
    #[prop(default = ToolbarOrientation::Horizontal)]
    orientation: ToolbarOrientation,

    /// Accessible label for the toolbar.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The toolbar items.
    children: ChildrenFn,
) -> impl IntoView {
    let current_item_id = RwSignal::new(None::<usize>);
    let first_item_id = RwSignal::new(None::<usize>);

    let ctx = ToolbarContext {
        orientation,
        current_item_id,
        first_item_id,
    };

    provide_context(ctx);

    let orientation_attr = match orientation {
        ToolbarOrientation::Horizontal => "horizontal",
        ToolbarOrientation::Vertical => "vertical",
    };

    view! {
        <div
            node_ref=node_ref
            role="toolbar"
            aria-label=aria_label
            class=class
            style=style
            data-radix-toolbar-root=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
    }
}

/// Button within a toolbar.
#[component]
pub fn ToolbarButton(
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,

    /// Whether the button is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the button.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The button content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ToolbarContext>()
        .expect("ToolbarButton must be used within ToolbarRoot");

    let item_id = TOOLBAR_ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Register this item with the toolbar
    register_toolbar_item(ctx, item_id, disabled.get_untracked());

    let on_click_handler = {
        let on_click = on_click.clone();
        move |ev: web_sys::MouseEvent| {
            if let Some(ref cb) = on_click {
                cb.run(ev);
            }
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        handle_toolbar_keydown(ev, ctx.orientation, ctx.current_item_id);
    };

    let on_focus = move |_| {
        ctx.current_item_id.set(Some(item_id));
    };

    let tabindex = move || get_tabindex(ctx, item_id);

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            tabindex=tabindex
            disabled=move || disabled.get()
            data-radix-toolbar-item=""
            data-radix-toolbar-item-id=item_id.to_string()
            data-radix-toolbar-button=""
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click_handler
            on:keydown=on_key_down
            on:focus=on_focus
        >
            {children()}
        </button>
    }
}

/// Separator between toolbar groups.
#[component]
pub fn ToolbarSeparator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the separator.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the separator element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,
) -> impl IntoView {
    let ctx = use_context::<ToolbarContext>()
        .expect("ToolbarSeparator must be used within ToolbarRoot");

    let orientation_attr = match ctx.orientation {
        ToolbarOrientation::Horizontal => "vertical",
        ToolbarOrientation::Vertical => "horizontal",
    };

    view! {
        <div
            node_ref=node_ref
            role="separator"
            aria-orientation=orientation_attr
            class=class
            style=style
            data-radix-toolbar-separator=""
            data-orientation=orientation_attr
        />
    }
}

/// Link within a toolbar.
#[component]
pub fn ToolbarLink(
    /// The link href.
    #[prop(into)]
    href: String,

    /// Link target.
    #[prop(optional, into)]
    target: Option<String>,

    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the link.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the link element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::A>,

    /// The link content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<ToolbarContext>()
        .expect("ToolbarLink must be used within ToolbarRoot");

    let item_id = TOOLBAR_ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Register this item - links are never disabled in this implementation
    register_toolbar_item(ctx, item_id, false);

    let on_click_handler = {
        let on_click = on_click.clone();
        move |ev: web_sys::MouseEvent| {
            if let Some(ref cb) = on_click {
                cb.run(ev);
            }
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        handle_toolbar_keydown(ev, ctx.orientation, ctx.current_item_id);
    };

    let on_focus = move |_| {
        ctx.current_item_id.set(Some(item_id));
    };

    let tabindex = move || get_tabindex(ctx, item_id);

    view! {
        <a
            node_ref=node_ref
            href=href
            target=target
            class=class
            style=style
            tabindex=tabindex
            data-radix-toolbar-item=""
            data-radix-toolbar-item-id=item_id.to_string()
            data-radix-toolbar-link=""
            on:click=on_click_handler
            on:keydown=on_key_down
            on:focus=on_focus
        >
            {children()}
        </a>
    }
}

/// Toggle group within a toolbar.
#[component]
pub fn ToolbarToggleGroup(
    /// Whether single or multiple items can be pressed.
    #[prop(default = ToolbarToggleType::Single)]
    toggle_type: ToolbarToggleType,

    /// Controlled value state.
    #[prop(optional)]
    value: Option<RwSignal<Vec<String>>>,

    /// Default value for uncontrolled mode.
    #[prop(optional, into)]
    default_value: Option<String>,

    /// Accessible label for the group.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the group.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the group element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The toggle items.
    children: ChildrenFn,
) -> impl IntoView {
    let _ = use_context::<ToolbarContext>()
        .expect("ToolbarToggleGroup must be used within ToolbarRoot");

    let value_signal = value.unwrap_or_else(|| {
        let default = default_value
            .map(|v| vec![v])
            .unwrap_or_default();
        RwSignal::new(default)
    });

    let group_ctx = ToolbarToggleGroupContext {
        toggle_type,
        value: value_signal,
    };

    provide_context(group_ctx);

    view! {
        <div
            node_ref=node_ref
            role="group"
            aria-label=aria_label
            class=class
            style=style
            data-radix-toolbar-toggle-group=""
        >
            {children()}
        </div>
    }
}

/// Toggle item within a toolbar toggle group.
#[component]
pub fn ToolbarToggleItem(
    /// The value that identifies this item.
    #[prop(into)]
    value: String,

    /// Whether this item is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Accessible label.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the item.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The item content.
    children: ChildrenFn,
) -> impl IntoView {
    let toolbar_ctx = use_context::<ToolbarContext>()
        .expect("ToolbarToggleItem must be used within ToolbarRoot");
    let group_ctx = use_context::<ToolbarToggleGroupContext>()
        .expect("ToolbarToggleItem must be used within ToolbarToggleGroup");

    let item_id = TOOLBAR_ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Register this item with the toolbar
    register_toolbar_item(toolbar_ctx, item_id, disabled.get_untracked());

    let item_value_for_click = value.clone();
    let item_value_for_state = value.clone();
    let item_value_for_pressed = value.clone();

    let on_click = {
        let group_ctx = group_ctx.clone();
        move |_| {
            if !disabled.get() {
                group_ctx.toggle(&item_value_for_click);
            }
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        handle_toolbar_keydown(ev, toolbar_ctx.orientation, toolbar_ctx.current_item_id);
    };

    let on_focus = move |_| {
        toolbar_ctx.current_item_id.set(Some(item_id));
    };

    let tabindex = move || get_tabindex(toolbar_ctx, item_id);

    let state_attr = {
        let group_ctx = group_ctx.clone();
        move || {
            if group_ctx.is_pressed(&item_value_for_state) {
                "on"
            } else {
                "off"
            }
        }
    };

    let aria_pressed = {
        let group_ctx = group_ctx.clone();
        move || group_ctx.is_pressed(&item_value_for_pressed).to_string()
    };

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            tabindex=tabindex
            aria-label=aria_label
            aria-pressed=aria_pressed
            disabled=move || disabled.get()
            data-radix-toolbar-item=""
            data-radix-toolbar-item-id=item_id.to_string()
            data-radix-toolbar-toggle-item=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
            on:keydown=on_key_down
            on:focus=on_focus
        >
            {children()}
        </button>
    }
}
