use leptos::html::Button;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Selection mode for the toggle group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupType {
    /// Only one item can be selected at a time. Selection can be cleared.
    #[default]
    Single,
    /// Multiple items can be selected simultaneously.
    Multiple,
}

/// Orientation of the toggle group for keyboard navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Context shared between ToggleGroup components.
#[derive(Clone, Copy)]
struct ToggleGroupContext {
    group_type: ToggleGroupType,
    value: RwSignal<Vec<String>>,
    disabled: Signal<bool>,
    #[allow(dead_code)] // Stored for data-orientation attribute on root
    orientation: ToggleGroupOrientation,
    /// Index of the currently focused item for roving tabindex
    focused_index: RwSignal<usize>,
    /// Total number of items registered
    item_count: RwSignal<usize>,
}

impl ToggleGroupContext {
    fn is_pressed(&self, item_value: &str) -> bool {
        self.value.get().contains(&item_value.to_string())
    }

    fn toggle(&self, item_value: &str) {
        match self.group_type {
            ToggleGroupType::Single => {
                self.value.update(|v| {
                    if v.contains(&item_value.to_string()) {
                        v.clear();
                    } else {
                        v.clear();
                        v.push(item_value.to_string());
                    }
                });
            }
            ToggleGroupType::Multiple => {
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

    fn move_focus(&self, delta: i32) -> usize {
        let count = self.item_count.get_untracked() as i32;
        if count == 0 {
            return 0;
        }
        let current = self.focused_index.get_untracked() as i32;
        let next = (current + delta).rem_euclid(count) as usize;
        self.focused_index.set(next);
        next
    }

    fn set_focus(&self, index: usize) -> usize {
        let count = self.item_count.get_untracked();
        let clamped = index.min(count.saturating_sub(1));
        self.focused_index.set(clamped);
        clamped
    }
}

/// Container for a group of toggle items with single or multiple selection.
#[component]
pub fn ToggleGroupRoot(
    /// Selection mode: single or multiple.
    #[prop(default = ToggleGroupType::Single)]
    group_type: ToggleGroupType,

    /// Controlled value state. For single mode, use a Vec with 0 or 1 element.
    /// For multiple mode, the Vec can have any number of elements.
    value: RwSignal<Vec<String>>,

    /// Whether all items in the group are disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Orientation affects keyboard navigation (arrow keys).
    #[prop(default = ToggleGroupOrientation::Horizontal)]
    orientation: ToggleGroupOrientation,

    /// Accessible label for the group.
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

    /// The toggle items.
    children: ChildrenFn,
) -> impl IntoView {
    let focused_index = RwSignal::new(0usize);
    let item_count = RwSignal::new(0usize);

    let ctx = ToggleGroupContext {
        group_type,
        value,
        disabled,
        orientation,
        focused_index,
        item_count,
    };

    provide_context(ctx);

    let orientation_attr = match orientation {
        ToggleGroupOrientation::Horizontal => "horizontal",
        ToggleGroupOrientation::Vertical => "vertical",
    };

    view! {
        <div
            node_ref=node_ref
            role="group"
            class=class
            style=style
            aria-label=aria_label
            data-radix-toggle-group-root=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
    }
}

/// An individual toggle button within a ToggleGroup.
#[component]
pub fn ToggleGroupItem(
    /// The value that identifies this item.
    #[prop(into)]
    value: String,

    /// Whether this specific item is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Accessible label for this item.
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
    let ctx = use_context::<ToggleGroupContext>()
        .expect("ToggleGroupItem must be used within ToggleGroupRoot");

    // Register this item and get its index
    let item_index = ctx.item_count.get_untracked();
    ctx.item_count.update(|c| *c += 1);

    let item_value = value.clone();
    let item_value_for_click = value.clone();

    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    let on_click = move |_| {
        if !is_disabled.get() {
            ctx.toggle(&item_value_for_click);
            ctx.focused_index.set(item_index);
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if is_disabled.get() {
            return;
        }

        let new_index = match ev.key().as_str() {
            // TODO: the react radix implementation does not respond to left/right keys
            // when in the vertical orientiation. Consider aligning, but I actually like
            // the current behavior where right = down and left = up
            "ArrowRight" | "ArrowDown" => Some(ctx.move_focus(1)),
            "ArrowLeft" | "ArrowUp" => Some(ctx.move_focus(-1)),
            "Home" => Some(ctx.set_focus(0)),
            "End" => Some(ctx.set_focus(usize::MAX)),
            _ => None,
        };

        let Some(target_index) = new_index else {
            return;
        };

        ev.prevent_default();

        // Find and focus the sibling button at target_index
        let Some(current_el) = ev.current_target() else {
            return;
        };
        let Ok(button) = current_el.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let Some(parent) = button.parent_element() else {
            return;
        };
        let Ok(buttons) = parent.query_selector_all("[data-radix-toggle-group-item]") else {
            return;
        };
        let Some(target) = buttons.get(target_index as u32) else {
            return;
        };
        let Ok(target_el) = target.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        _ = target_el.focus();
    };

    let on_focus = move |_| {
        ctx.focused_index.set(item_index);
    };

    let state_attr = {
        let item_value = item_value.clone();
        move || {
            if ctx.is_pressed(&item_value) {
                "on"
            } else {
                "off"
            }
        }
    };

    let pressed_attr = {
        let item_value = item_value.clone();
        move || ctx.is_pressed(&item_value).to_string()
    };

    // Roving tabindex: only the focused item has tabindex=0
    let tabindex = move || {
        if is_disabled.get() {
            -1
        } else if ctx.focused_index.get() == item_index {
            0
        } else {
            -1
        }
    };

    view! {
        <button
            node_ref=node_ref
            type="button"
            class=class
            style=style
            tabindex=tabindex
            aria-label=aria_label
            aria-pressed=pressed_attr
            disabled=move || is_disabled.get()
            data-radix-toggle-group-item=""
            data-state=state_attr
            data-disabled=move || is_disabled.get().then_some("")
            on:click=on_click
            on:keydown=on_key_down
            on:focus=on_focus
        >
            {children()}
        </button>
    }
}
