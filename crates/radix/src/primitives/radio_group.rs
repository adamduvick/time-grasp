use leptos::html::Button;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Orientation of the radio group for keyboard navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Context shared between RadioGroup components.
#[derive(Clone, Copy)]
struct RadioGroupContext {
    value: RwSignal<String>,
    disabled: Signal<bool>,
    #[allow(dead_code)] // Used for data-orientation attribute
    orientation: RadioGroupOrientation,
    focused_index: RwSignal<usize>,
    item_count: RwSignal<usize>,
}

impl RadioGroupContext {
    fn is_checked(&self, item_value: &str) -> bool {
        self.value.get() == item_value
    }

    fn select(&self, item_value: &str) {
        self.value.set(item_value.to_string());
    }
}

/// Context for individual radio items to access their checked state.
#[derive(Clone, Copy)]
struct RadioItemContext {
    value: StoredValue<String>,
    disabled: Signal<bool>,
}

/// Root container for a radio group with single selection.
#[component]
pub fn RadioGroupRoot(
    /// Controlled selected value.
    value: RwSignal<String>,

    /// Whether all items in the group are disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Orientation affects keyboard navigation (arrow keys).
    #[prop(default = RadioGroupOrientation::Vertical)]
    orientation: RadioGroupOrientation,

    /// Accessible label for the group.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// Form field name for submission.
    #[prop(optional, into)]
    name: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The radio items.
    children: ChildrenFn,
) -> impl IntoView {
    let focused_index = RwSignal::new(0usize);
    let item_count = RwSignal::new(0usize);

    let ctx = RadioGroupContext {
        value,
        disabled,
        orientation,
        focused_index,
        item_count,
    };

    provide_context(ctx);

    let orientation_attr = match orientation {
        RadioGroupOrientation::Horizontal => "horizontal",
        RadioGroupOrientation::Vertical => "vertical",
    };

    // Hidden input for form submission
    let input_name = name.clone();

    view! {
        <div
            node_ref=node_ref
            role="radiogroup"
            class=class
            style=style
            aria-label=aria_label
            data-radix-radio-group-root=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
        {move || {
            if let Some(ref name) = input_name {
                let current_value = value.get();
                Some(view! {
                    <input
                        type="hidden"
                        name=name.clone()
                        value=current_value
                    />
                })
            } else {
                None
            }
        }}
    }
}

/// An individual radio button within a RadioGroup.
#[component]
pub fn RadioGroupItem(
    /// The value that identifies this item.
    #[prop(into)]
    value: String,

    /// Whether this specific item is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// HTML id attribute for label association.
    #[prop(optional, into)]
    id: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the item.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The item content (typically RadioGroupIndicator).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<RadioGroupContext>()
        .expect("RadioGroupItem must be used within RadioGroupRoot");

    // Register this item and get its index
    let item_index = ctx.item_count.get_untracked();
    ctx.item_count.update(|c| *c += 1);

    let item_value = value.clone();
    let item_value_for_click = value.clone();
    let item_value_stored = StoredValue::new(value.clone());

    let is_disabled = Signal::derive(move || disabled.get() || ctx.disabled.get());

    // Provide context for the indicator
    let item_ctx = RadioItemContext {
        value: item_value_stored,
        disabled: is_disabled,
    };
    provide_context(item_ctx);

    let on_click = move |_| {
        if !is_disabled.get() {
            ctx.select(&item_value_for_click);
            ctx.focused_index.set(item_index);
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if is_disabled.get() {
            return;
        }

        let direction = match ev.key().as_str() {
            "ArrowRight" | "ArrowDown" => Some(1i32),
            "ArrowLeft" | "ArrowUp" => Some(-1i32),
            "Home" => Some(0i32),    // Special: go to first
            "End" => Some(i32::MAX), // Special: go to last
            _ => None,
        };

        let Some(dir) = direction else {
            return;
        };

        ev.prevent_default();

        // Find all radio items in the group
        let Some(current_el) = ev.current_target() else {
            return;
        };
        let Ok(button) = current_el.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let Some(parent) = button.parent_element() else {
            return;
        };
        let Some(grandparent) = parent.parent_element() else {
            return;
        };
        let Ok(buttons) = grandparent.query_selector_all("[data-radix-radio-group-item]") else {
            return;
        };

        let count = buttons.length() as i32;
        if count == 0 {
            return;
        }

        // Find current index
        let current_index = ctx.focused_index.get_untracked() as i32;

        // Find next enabled item
        let target_index = if dir == 0 {
            // Home: find first enabled
            (0..count).find(|&i| {
                buttons
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else if dir == i32::MAX {
            // End: find last enabled
            (0..count).rev().find(|&i| {
                buttons
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else {
            // Arrow: find next enabled in direction, wrapping
            (1..=count).find_map(|offset| {
                let i = (current_index + dir * offset).rem_euclid(count);
                let is_enabled = buttons
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false);
                if is_enabled { Some(i) } else { None }
            })
        };

        let Some(target_idx) = target_index else {
            return;
        };

        ctx.focused_index.set(target_idx as usize);

        let Some(target) = buttons.get(target_idx as u32) else {
            return;
        };
        let Ok(target_el) = target.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        _ = target_el.focus();
        _ = target_el.click(); // Select on focus like native radio
    };

    let on_focus = move |_| {
        ctx.focused_index.set(item_index);
    };

    let state_attr = {
        let item_value = item_value.clone();
        move || {
            if ctx.is_checked(&item_value) {
                "checked"
            } else {
                "unchecked"
            }
        }
    };

    // Roving tabindex: only the checked item (or first if none checked) has tabindex=0
    let tabindex = {
        let item_value = item_value.clone();
        move || {
            if is_disabled.get() {
                -1
            } else if ctx.is_checked(&item_value) {
                0
            } else if ctx.focused_index.get() == item_index {
                0
            } else {
                -1
            }
        }
    };

    let aria_checked = {
        let item_value = item_value.clone();
        move || ctx.is_checked(&item_value).to_string()
    };

    view! {
        <button
            node_ref=node_ref
            type="button"
            role="radio"
            id=id
            class=class
            style=style
            tabindex=tabindex
            aria-checked=aria_checked
            disabled=move || is_disabled.get()
            data-radix-radio-group-item=""
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

/// Indicator that renders only when the radio item is checked.
#[component]
pub fn RadioGroupIndicator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the indicator element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the indicator element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,
) -> impl IntoView {
    let group_ctx = use_context::<RadioGroupContext>()
        .expect("RadioGroupIndicator must be used within RadioGroupRoot");
    let item_ctx = use_context::<RadioItemContext>()
        .expect("RadioGroupIndicator must be used within RadioGroupItem");

    let is_checked = move || group_ctx.value.get() == item_ctx.value.get_value();

    view! {
        <Show when=is_checked>
            <span
                node_ref=node_ref
                class=class.clone()
                style=style.clone()
                data-radix-radio-group-indicator=""
                data-state="checked"
                data-disabled=move || item_ctx.disabled.get().then_some("")
            />
        </Show>
    }
}
