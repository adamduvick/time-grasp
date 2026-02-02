use leptos::html::Button;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

/// Accordion mode - single or multiple items can be open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionType {
    /// Only one item can be open at a time.
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

/// Orientation of the accordion for keyboard navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionOrientation {
    #[default]
    Vertical,
    Horizontal,
}

static ACCORDION_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Context shared between Accordion components.
#[derive(Clone, Copy)]
struct AccordionContext {
    accordion_type: AccordionType,
    value: RwSignal<Vec<String>>,
    collapsible: bool,
    orientation: AccordionOrientation,
    base_id: RwSignal<String>,
}

impl AccordionContext {
    fn is_open(&self, item_value: &str) -> bool {
        self.value.get().contains(&item_value.to_string())
    }

    fn toggle(&self, item_value: &str) {
        match self.accordion_type {
            AccordionType::Single => {
                self.value.update(|v| {
                    if v.contains(&item_value.to_string()) {
                        if self.collapsible {
                            v.clear();
                        }
                        // If not collapsible, do nothing (keep it open)
                    } else {
                        v.clear();
                        v.push(item_value.to_string());
                    }
                });
            }
            AccordionType::Multiple => {
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

    fn trigger_id(&self, item_value: &str) -> String {
        format!("{}-trigger-{}", self.base_id.get(), item_value)
    }

    fn content_id(&self, item_value: &str) -> String {
        format!("{}-content-{}", self.base_id.get(), item_value)
    }
}

/// Context for individual accordion items.
#[derive(Clone, Copy)]
struct AccordionItemContext {
    value: StoredValue<String>,
    disabled: Signal<bool>,
}

/// Root container for an accordion.
#[component]
pub fn AccordionRoot(
    /// Whether single or multiple items can be open.
    #[prop(default = AccordionType::Single)]
    accordion_type: AccordionType,

    /// Controlled value state. For single mode, use a Vec with 0 or 1 element.
    value: RwSignal<Vec<String>>,

    /// Whether all items can be closed in single mode (default: false).
    #[prop(default = false)]
    collapsible: bool,

    /// Orientation affects keyboard navigation.
    #[prop(default = AccordionOrientation::Vertical)]
    orientation: AccordionOrientation,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The accordion items.
    children: ChildrenFn,
) -> impl IntoView {
    let id = ACCORDION_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let base_id = RwSignal::new(format!("accordion-{}", id));

    let ctx = AccordionContext {
        accordion_type,
        value,
        collapsible,
        orientation,
        base_id,
    };

    provide_context(ctx);

    let orientation_attr = match orientation {
        AccordionOrientation::Vertical => "vertical",
        AccordionOrientation::Horizontal => "horizontal",
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-accordion-root=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
    }
}

/// Container for a single accordion item.
#[component]
pub fn AccordionItem(
    /// The value that identifies this item.
    #[prop(into)]
    value: String,

    /// Whether this item is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the item element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the item element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The item content (header and content).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionItem must be used within AccordionRoot");

    let item_value = value.clone();
    let item_value_stored = StoredValue::new(value);

    let item_ctx = AccordionItemContext {
        value: item_value_stored,
        disabled,
    };
    provide_context(item_ctx);

    let state_attr = move || {
        if ctx.is_open(&item_value) {
            "open"
        } else {
            "closed"
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-accordion-item=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
        >
            {children()}
        </div>
    }
}

/// Header container for the accordion trigger.
#[component]
pub fn AccordionHeader(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the header element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the header element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The header content (typically AccordionTrigger).
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-accordion-header=""
        >
            {children()}
        </div>
    }
}

/// Trigger button that toggles the accordion item.
#[component]
pub fn AccordionTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the trigger element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The trigger content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionTrigger must be used within AccordionRoot");
    let item_ctx = use_context::<AccordionItemContext>()
        .expect("AccordionTrigger must be used within AccordionItem");

    let item_value = item_ctx.value.get_value();
    let item_value_for_click = item_value.clone();
    let item_value_for_state = item_value.clone();
    let item_value_for_expanded = item_value.clone();
    let item_value_for_trigger_id = item_value.clone();
    let item_value_for_content_id = item_value.clone();

    let on_click = move |_| {
        if !item_ctx.disabled.get() {
            ctx.toggle(&item_value_for_click);
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if item_ctx.disabled.get() {
            return;
        }

        let orientation = ctx.orientation;

        let direction = match ev.key().as_str() {
            "ArrowDown" if orientation == AccordionOrientation::Vertical => Some(1i32),
            "ArrowUp" if orientation == AccordionOrientation::Vertical => Some(-1i32),
            "ArrowRight" if orientation == AccordionOrientation::Horizontal => Some(1i32),
            "ArrowLeft" if orientation == AccordionOrientation::Horizontal => Some(-1i32),
            "Home" => Some(0i32),
            "End" => Some(i32::MAX),
            _ => None,
        };

        let Some(dir) = direction else {
            return;
        };

        ev.prevent_default();

        // Find all accordion triggers
        let Some(current_el) = ev.current_target() else {
            return;
        };
        let Ok(button) = current_el.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };

        // Navigate up to find the accordion root
        let mut element = button.parent_element();
        let mut root: Option<web_sys::Element> = None;
        while let Some(el) = element {
            if el.has_attribute("data-radix-accordion-root") {
                root = Some(el);
                break;
            }
            element = el.parent_element();
        }

        let Some(root_el) = root else {
            return;
        };

        let Ok(triggers) = root_el.query_selector_all("[data-radix-accordion-trigger]") else {
            return;
        };

        let count = triggers.length() as i32;
        if count == 0 {
            return;
        }

        // Find current index
        let mut current_index = 0i32;
        for i in 0..count {
            if let Some(el) = triggers.get(i as u32) {
                if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                    if html_el == button {
                        current_index = i;
                        break;
                    }
                }
            }
        }

        // Find next enabled trigger
        let target_index = if dir == 0 {
            (0..count).find(|&i| {
                triggers
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else if dir == i32::MAX {
            (0..count).rev().find(|&i| {
                triggers
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else {
            (1..=count).find_map(|offset| {
                let i = (current_index + dir * offset).rem_euclid(count);
                let is_enabled = triggers
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false);
                if is_enabled { Some(i) } else { None }
            })
        };

        let Some(target_idx) = target_index else {
            return;
        };

        let Some(target) = triggers.get(target_idx as u32) else {
            return;
        };
        let Ok(target_el) = target.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        _ = target_el.focus();
    };

    let state_attr = move || {
        if ctx.is_open(&item_value_for_state) {
            "open"
        } else {
            "closed"
        }
    };

    let aria_expanded = move || ctx.is_open(&item_value_for_expanded).to_string();

    let trigger_id = ctx.trigger_id(&item_value_for_trigger_id);
    let content_id = ctx.content_id(&item_value_for_content_id);

    view! {
        <button
            node_ref=node_ref
            type="button"
            id=trigger_id
            class=class
            style=style
            aria-expanded=aria_expanded
            aria-controls=content_id
            disabled=move || item_ctx.disabled.get()
            data-radix-accordion-trigger=""
            data-state=state_attr
            data-disabled=move || item_ctx.disabled.get().then_some("")
            on:click=on_click
            on:keydown=on_key_down
        >
            {children()}
        </button>
    }
}

/// Content panel for an accordion item.
#[component]
pub fn AccordionContent(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the content element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<AccordionContext>()
        .expect("AccordionContent must be used within AccordionRoot");
    let item_ctx = use_context::<AccordionItemContext>()
        .expect("AccordionContent must be used within AccordionItem");

    let item_value = item_ctx.value.get_value();
    let item_value_for_state = item_value.clone();
    let item_value_for_hidden = item_value.clone();
    let item_value_for_id = item_value.clone();
    let item_value_for_labelledby = item_value.clone();

    let state_attr = move || {
        if ctx.is_open(&item_value_for_state) {
            "open"
        } else {
            "closed"
        }
    };

    let is_hidden = move || !ctx.is_open(&item_value_for_hidden);

    let content_id = ctx.content_id(&item_value_for_id);
    let trigger_id = ctx.trigger_id(&item_value_for_labelledby);

    view! {
        <div
            node_ref=node_ref
            role="region"
            id=content_id
            class=class.clone()
            style=style.clone()
            aria-labelledby=trigger_id
            hidden=is_hidden
            data-radix-accordion-content=""
            data-state=state_attr
            data-disabled=move || item_ctx.disabled.get().then_some("")
        >
            {children()}
        </div>
    }
}
