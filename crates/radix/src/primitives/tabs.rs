use leptos::html::Button;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

/// Orientation of the tabs for keyboard navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Activation mode for tabs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsActivationMode {
    /// Focus automatically activates the tab.
    #[default]
    Automatic,
    /// Tab must be explicitly activated with Enter/Space.
    Manual,
}

static TABS_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Context shared between Tabs components.
#[derive(Clone, Copy)]
struct TabsContext {
    value: RwSignal<String>,
    orientation: TabsOrientation,
    activation_mode: TabsActivationMode,
    base_id: RwSignal<String>,
}

impl TabsContext {
    fn is_active(&self, tab_value: &str) -> bool {
        self.value.get() == tab_value
    }

    fn activate(&self, tab_value: &str) {
        self.value.set(tab_value.to_string());
    }

    fn trigger_id(&self, tab_value: &str) -> String {
        format!("{}-trigger-{}", self.base_id.get(), tab_value)
    }

    fn content_id(&self, tab_value: &str) -> String {
        format!("{}-content-{}", self.base_id.get(), tab_value)
    }
}

/// Root container for tabbed content.
#[component]
pub fn TabsRoot(
    /// Controlled active tab value.
    value: RwSignal<String>,

    /// Orientation affects keyboard navigation.
    #[prop(default = TabsOrientation::Horizontal)]
    orientation: TabsOrientation,

    /// Whether tabs activate on focus or require explicit activation.
    #[prop(default = TabsActivationMode::Automatic)]
    activation_mode: TabsActivationMode,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The tabs content (TabsList and TabsContent components).
    children: ChildrenFn,
) -> impl IntoView {
    let id = TABS_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let base_id = RwSignal::new(format!("tabs-{}", id));

    let ctx = TabsContext {
        value,
        orientation,
        activation_mode,
        base_id,
    };

    provide_context(ctx);

    let orientation_attr = match orientation {
        TabsOrientation::Horizontal => "horizontal",
        TabsOrientation::Vertical => "vertical",
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-tabs-root=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
    }
}

/// Container for tab triggers.
#[component]
pub fn TabsList(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the list element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the list element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The tab triggers.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<TabsContext>().expect("TabsList must be used within TabsRoot");

    let orientation_attr = match ctx.orientation {
        TabsOrientation::Horizontal => "horizontal",
        TabsOrientation::Vertical => "vertical",
    };

    view! {
        <div
            node_ref=node_ref
            role="tablist"
            class=class
            style=style
            aria-orientation=orientation_attr
            data-radix-tabs-list=""
            data-orientation=orientation_attr
        >
            {children()}
        </div>
    }
}

/// Individual tab trigger button.
#[component]
pub fn TabsTrigger(
    /// The value that identifies this tab.
    #[prop(into)]
    value: String,

    /// Whether this tab is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

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
    let ctx = use_context::<TabsContext>().expect("TabsTrigger must be used within TabsRoot");

    let tab_value = value.clone();
    let tab_value_for_click = value.clone();
    let tab_value_for_id = value.clone();
    let tab_value_for_controls = value.clone();

    let on_click = move |_| {
        if !disabled.get() {
            ctx.activate(&tab_value_for_click);
        }
    };

    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if disabled.get() {
            return;
        }

        let orientation = ctx.orientation;

        let direction = match ev.key().as_str() {
            "ArrowRight" if orientation == TabsOrientation::Horizontal => Some(1i32),
            "ArrowLeft" if orientation == TabsOrientation::Horizontal => Some(-1i32),
            "ArrowDown" if orientation == TabsOrientation::Vertical => Some(1i32),
            "ArrowUp" if orientation == TabsOrientation::Vertical => Some(-1i32),
            "Home" => Some(0i32),
            "End" => Some(i32::MAX),
            _ => None,
        };

        let Some(dir) = direction else {
            return;
        };

        ev.prevent_default();

        // Find all tab triggers
        let Some(current_el) = ev.current_target() else {
            return;
        };
        let Ok(button) = current_el.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let Some(parent) = button.parent_element() else {
            return;
        };
        let Ok(triggers) = parent.query_selector_all("[data-radix-tabs-trigger]") else {
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
                if el == button.clone().into() {
                    current_index = i;
                    break;
                }
            }
        }

        // Find next enabled tab
        let target_index = if dir == 0 {
            // Home: find first enabled
            (0..count).find(|&i| {
                triggers
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else if dir == i32::MAX {
            // End: find last enabled
            (0..count).rev().find(|&i| {
                triggers
                    .get(i as u32)
                    .and_then(|el| el.dyn_ref::<web_sys::HtmlButtonElement>().map(|b| !b.disabled()))
                    .unwrap_or(false)
            })
        } else {
            // Arrow: find next enabled in direction, wrapping
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

        // In automatic mode, also activate the tab
        if ctx.activation_mode == TabsActivationMode::Automatic {
            _ = target_el.click();
        }
    };

    let state_attr = {
        let tab_value = tab_value.clone();
        move || if ctx.is_active(&tab_value) { "active" } else { "inactive" }
    };

    let trigger_id = ctx.trigger_id(&tab_value_for_id);
    let content_id = ctx.content_id(&tab_value_for_controls);

    // Roving tabindex
    let tabindex = {
        let tab_value = tab_value.clone();
        move || {
            if disabled.get() {
                -1
            } else if ctx.is_active(&tab_value) {
                0
            } else {
                -1
            }
        }
    };

    let aria_selected = {
        let tab_value = tab_value.clone();
        move || ctx.is_active(&tab_value).to_string()
    };

    view! {
        <button
            node_ref=node_ref
            type="button"
            role="tab"
            id=trigger_id
            class=class
            style=style
            tabindex=tabindex
            aria-selected=aria_selected
            aria-controls=content_id
            disabled=move || disabled.get()
            data-radix-tabs-trigger=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
            on:keydown=on_key_down
        >
            {children()}
        </button>
    }
}

/// Content panel for a tab.
#[component]
pub fn TabsContent(
    /// The value that identifies which tab this content belongs to.
    #[prop(into)]
    value: String,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the content element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the content element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Div>,

    /// The panel content.
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<TabsContext>().expect("TabsContent must be used within TabsRoot");

    let tab_value = value.clone();
    let tab_value_for_id = value.clone();
    let tab_value_for_labelledby = value.clone();

    let is_active = {
        let tab_value = tab_value.clone();
        move || ctx.is_active(&tab_value)
    };

    let state_attr = {
        let tab_value = tab_value.clone();
        move || if ctx.is_active(&tab_value) { "active" } else { "inactive" }
    };

    let content_id = ctx.content_id(&tab_value_for_id);
    let trigger_id = ctx.trigger_id(&tab_value_for_labelledby);

    view! {
        <div
            node_ref=node_ref
            role="tabpanel"
            id=content_id
            class=class.clone()
            style=style.clone()
            aria-labelledby=trigger_id
            hidden=move || !is_active()
            tabindex=0
            data-radix-tabs-content=""
            data-state=state_attr
        >
            {children()}
        </div>
    }
}
