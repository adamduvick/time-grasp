use leptos::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::JsCast;

use crate::{
    MenuAnchor, MenuArrow, MenuCheckboxItem, MenuContent, MenuGroup, MenuItem, MenuItemIndicator,
    MenuLabel, MenuPortal, MenuRadioGroup, MenuRadioItem, MenuRoot, MenuSeparator, PopperAlign,
    PopperSide,
};

// Counter for generating unique menubar IDs
static MENUBAR_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Context shared across the entire menubar.
#[derive(Clone, Copy)]
struct MenubarContext {
    /// Currently open menu (by index), None if all closed
    open_menu: RwSignal<Option<u32>>,
    /// List of registered menu IDs for keyboard navigation
    menu_ids: RwSignal<Vec<u32>>,
    /// Whether to open menus on hover (after one is already open)
    open_on_hover: bool,
    /// Direction for layout
    dir: StoredValue<String>,
}

/// Context for individual menu within the menubar.
#[derive(Clone, Copy)]
struct MenubarMenuContext {
    menu_id: u32,
    open: RwSignal<bool>,
    /// Trigger element ID for focus return
    trigger_id: StoredValue<String>,
}

/// Root component for a menubar.
#[component]
pub fn MenubarRoot(
    /// Whether menus open on hover after one is already open.
    #[prop(default = true)]
    open_on_hover: bool,

    /// Text direction (ltr or rtl).
    #[prop(into, default = "ltr".to_string())]
    dir: String,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The menubar content.
    children: Children,
) -> impl IntoView {
    let open_menu = RwSignal::new(None::<u32>);
    let menu_ids = RwSignal::new(Vec::<u32>::new());

    let ctx = MenubarContext {
        open_menu,
        menu_ids,
        open_on_hover,
        dir: StoredValue::new(dir),
    };

    provide_context(ctx);

    view! {
        <div
            role="menubar"
            class=class
            style=style
            data-radix-menubar-root=""
            dir=ctx.dir.get_value()
        >
            {children()}
        </div>
    }
}

/// Individual menu within the menubar.
#[component]
pub fn MenubarMenu(
    /// The menu content.
    children: Children,
) -> impl IntoView {
    let menubar_ctx =
        use_context::<MenubarContext>().expect("MenubarMenu must be used within MenubarRoot");

    // Generate unique ID for this menu
    let menu_id = MENUBAR_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

    // Register this menu
    menubar_ctx.menu_ids.update(|ids| {
        if !ids.contains(&menu_id) {
            ids.push(menu_id);
        }
    });

    let open = RwSignal::new(false);
    let trigger_id = StoredValue::new(format!("menubar-trigger-{}", menu_id));

    // Sync open state with menubar context
    Effect::new(move || {
        let is_open = menubar_ctx.open_menu.get() == Some(menu_id);
        open.set(is_open);
    });

    let on_open_change = Callback::new(move |is_open: bool| {
        if is_open {
            menubar_ctx.open_menu.set(Some(menu_id));
        } else {
            // Only close if this menu is the one that's open
            if menubar_ctx.open_menu.get() == Some(menu_id) {
                menubar_ctx.open_menu.set(None);
            }
        }
    });

    let ctx = MenubarMenuContext {
        menu_id,
        open,
        trigger_id,
    };

    provide_context(ctx);

    view! {
        <MenuRoot open=open on_open_change=on_open_change modal=true>
            {children()}
        </MenuRoot>
    }
}

/// Trigger button for a menu in the menubar.
#[component]
pub fn MenubarTrigger(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this trigger is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// The trigger content.
    children: Children,
) -> impl IntoView {
    let menubar_ctx =
        use_context::<MenubarContext>().expect("MenubarTrigger must be used within MenubarRoot");
    let menu_ctx =
        use_context::<MenubarMenuContext>().expect("MenubarTrigger must be used within MenubarMenu");

    let menu_id = menu_ctx.menu_id;
    let open = menu_ctx.open;
    let trigger_id = menu_ctx.trigger_id;

    let on_click = move |_: web_sys::MouseEvent| {
        if disabled {
            return;
        }
        if open.get() {
            menubar_ctx.open_menu.set(None);
        } else {
            menubar_ctx.open_menu.set(Some(menu_id));
        }
    };

    let on_pointer_enter = move |_: web_sys::PointerEvent| {
        if disabled {
            return;
        }
        // Open on hover if another menu is already open
        if menubar_ctx.open_on_hover && menubar_ctx.open_menu.get().is_some() {
            menubar_ctx.open_menu.set(Some(menu_id));
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled {
            return;
        }

        let key = ev.key();
        let menu_ids = menubar_ctx.menu_ids.get();
        let current_idx = menu_ids.iter().position(|&id| id == menu_id);

        match key.as_str() {
            "ArrowDown" | "Enter" | " " => {
                ev.prevent_default();
                menubar_ctx.open_menu.set(Some(menu_id));
            }
            "ArrowRight" => {
                ev.prevent_default();
                if let Some(idx) = current_idx {
                    let next_idx = (idx + 1) % menu_ids.len();
                    let next_id = menu_ids[next_idx];
                    // Focus the next trigger
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let next_trigger_id = format!("menubar-trigger-{}", next_id);
                        if let Some(el) = document.get_element_by_id(&next_trigger_id) {
                            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_el.focus();
                            }
                        }
                    }
                }
            }
            "ArrowLeft" => {
                ev.prevent_default();
                if let Some(idx) = current_idx {
                    let prev_idx = if idx == 0 { menu_ids.len() - 1 } else { idx - 1 };
                    let prev_id = menu_ids[prev_idx];
                    // Focus the previous trigger
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        let prev_trigger_id = format!("menubar-trigger-{}", prev_id);
                        if let Some(el) = document.get_element_by_id(&prev_trigger_id) {
                            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_el.focus();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let data_state = move || if open.get() { "open" } else { "closed" };

    view! {
        <MenuAnchor>
            <button
                id=trigger_id.get_value()
                type="button"
                role="menuitem"
                class=class
                style=style
                disabled=disabled
                aria-haspopup="menu"
                aria-expanded=move || open.get().to_string()
                data-radix-menubar-trigger=""
                data-state=data_state
                data-disabled=disabled.then_some("")
                on:click=on_click
                on:pointerenter=on_pointer_enter
                on:keydown=on_keydown
            >
                {children()}
            </button>
        </MenuAnchor>
    }
}

/// Portal for menu content - delegates to MenuPortal.
#[component]
pub fn MenubarPortal(
    /// The portal content.
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <MenuPortal>
            {children()}
        </MenuPortal>
    }
}

/// Content of a menu dropdown - delegates to MenuContent.
#[component]
pub fn MenubarContent(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Alignment relative to trigger.
    #[prop(default = MenubarAlign::Start)]
    align: MenubarAlign,

    /// Side offset in pixels.
    #[prop(default = 0)]
    side_offset: i32,

    /// Align offset in pixels.
    #[prop(default = 0)]
    align_offset: i32,

    /// The menu content.
    children: ChildrenFn,
) -> impl IntoView {
    let menubar_ctx =
        use_context::<MenubarContext>().expect("MenubarContent must be used within MenubarRoot");
    let menu_ctx =
        use_context::<MenubarMenuContext>().expect("MenubarContent must be used within MenubarMenu");

    let trigger_id = menu_ctx.trigger_id;
    let menu_id = menu_ctx.menu_id;

    // Convert MenubarAlign to PopperAlign
    let popper_align = match align {
        MenubarAlign::Start => PopperAlign::Start,
        MenubarAlign::Center => PopperAlign::Center,
        MenubarAlign::End => PopperAlign::End,
    };

    // Handle arrow keys to navigate between menus
    let on_keydown = Callback::new(move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        match key.as_str() {
            "ArrowRight" => {
                ev.prevent_default();
                let menu_ids = menubar_ctx.menu_ids.get();
                if let Some(idx) = menu_ids.iter().position(|&id| id == menu_id) {
                    let next_idx = (idx + 1) % menu_ids.len();
                    let next_id = menu_ids[next_idx];
                    menubar_ctx.open_menu.set(Some(next_id));
                }
            }
            "ArrowLeft" => {
                ev.prevent_default();
                let menu_ids = menubar_ctx.menu_ids.get();
                if let Some(idx) = menu_ids.iter().position(|&id| id == menu_id) {
                    let prev_idx = if idx == 0 { menu_ids.len() - 1 } else { idx - 1 };
                    let prev_id = menu_ids[prev_idx];
                    menubar_ctx.open_menu.set(Some(prev_id));
                }
            }
            "Escape" => {
                // Menu handles Escape too, but we want to return focus to trigger
                ev.prevent_default();
                menubar_ctx.open_menu.set(None);
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let id = trigger_id.get_value();
                    if let Some(el) = document.get_element_by_id(&id) {
                        if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                            let _ = html_el.focus();
                        }
                    }
                }
            }
            _ => {}
        }
    });

    let children = StoredValue::new(children);

    view! {
        <MenuContent
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            side=Signal::derive(|| PopperSide::Bottom)
            side_offset=Signal::derive(move || side_offset)
            align=Signal::derive(move || popper_align)
            align_offset=Signal::derive(move || align_offset)
            on_keydown=on_keydown
        >
            {children.with_value(|c| c())}
        </MenuContent>
    }
}

/// Individual menu item - delegates to MenuItem.
#[component]
pub fn MenubarItem(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Callback when the item is selected.
    #[prop(optional)]
    on_select: Option<Callback<()>>,

    /// Text label for typeahead (if different from children text).
    #[prop(optional, into)]
    text_value: Option<String>,

    /// The item content.
    children: Children,
) -> impl IntoView {
    // Create a callback that forwards to the optional callback
    let wrapped_on_select = Callback::new(move |_: ()| {
        if let Some(cb) = on_select {
            cb.run(());
        }
    });

    view! {
        <MenuItem
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            disabled=disabled
            on_select=wrapped_on_select
            text_value=text_value.unwrap_or_default()
        >
            {children()}
        </MenuItem>
    }
}

/// Separator between menu items - delegates to MenuSeparator.
#[component]
pub fn MenubarSeparator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    view! {
        <MenuSeparator class=class.unwrap_or_default() style=style.unwrap_or_default() />
    }
}

/// Label for grouping menu items - delegates to MenuLabel.
#[component]
pub fn MenubarLabel(
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
        <MenuLabel class=class.unwrap_or_default() style=style.unwrap_or_default()>
            {children()}
        </MenuLabel>
    }
}

/// Group of menu items - delegates to MenuGroup.
#[component]
pub fn MenubarGroup(
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
        <MenuGroup class=class.unwrap_or_default() style=style.unwrap_or_default()>
            {children()}
        </MenuGroup>
    }
}

/// Checkbox item within a menu - delegates to MenuCheckboxItem.
#[component]
pub fn MenubarCheckboxItem(
    /// Whether the checkbox is checked.
    #[prop(into)]
    checked: RwSignal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Callback when checked state changes.
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,

    /// Text label for typeahead (if different from children text).
    #[prop(optional, into)]
    text_value: Option<String>,

    /// The item content.
    children: Children,
) -> impl IntoView {
    // Create a callback that forwards to the optional callback
    let wrapped_on_checked_change = Callback::new(move |v: bool| {
        if let Some(cb) = on_checked_change {
            cb.run(v);
        }
    });

    view! {
        <MenuCheckboxItem
            checked=checked
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            disabled=disabled
            on_checked_change=wrapped_on_checked_change
            text_value=text_value.unwrap_or_default()
        >
            {children()}
        </MenuCheckboxItem>
    }
}

/// Radio group within a menu - delegates to MenuRadioGroup.
#[component]
pub fn MenubarRadioGroup(
    /// The current value.
    #[prop(into)]
    value: RwSignal<String>,

    /// Callback when value changes.
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,

    /// The group content.
    children: Children,
) -> impl IntoView {
    // Create a callback that forwards to the optional callback
    let wrapped_on_value_change = Callback::new(move |v: String| {
        if let Some(cb) = on_value_change {
            cb.run(v);
        }
    });

    view! {
        <MenuRadioGroup value=value on_value_change=wrapped_on_value_change>
            {children()}
        </MenuRadioGroup>
    }
}

/// Radio item within a menu - delegates to MenuRadioItem.
#[component]
pub fn MenubarRadioItem(
    /// The value of this radio item.
    #[prop(into)]
    value: String,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// Whether this item is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Text label for typeahead.
    #[prop(optional, into)]
    text_value: Option<String>,

    /// The item content.
    children: Children,
) -> impl IntoView {
    view! {
        <MenuRadioItem
            value=value
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            disabled=disabled
            text_value=text_value.unwrap_or_default()
        >
            {children()}
        </MenuRadioItem>
    }
}

/// Indicator for checkbox/radio items - delegates to MenuItemIndicator.
#[component]
pub fn MenubarItemIndicator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles.
    #[prop(optional, into)]
    style: Option<String>,

    /// The indicator content (e.g., checkmark icon).
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <MenuItemIndicator class=class.unwrap_or_default() style=style.unwrap_or_default()>
            {children.with_value(|c| c())}
        </MenuItemIndicator>
    }
}

/// Arrow for menu content - delegates to MenuArrow.
#[component]
pub fn MenubarArrow(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Arrow width in pixels.
    #[prop(default = 10)]
    width: u32,

    /// Arrow height in pixels.
    #[prop(default = 5)]
    height: u32,
) -> impl IntoView {
    view! {
        <MenuArrow
            class=class.unwrap_or_default()
            width=width
            height=height
        />
    }
}

/// Alignment options for menu content.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MenubarAlign {
    #[default]
    Start,
    Center,
    End,
}
