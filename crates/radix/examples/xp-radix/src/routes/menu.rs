use leptos::prelude::*;
use radix::{
    MenuAnchor, MenuArrow, MenuCheckboxItem, MenuContent, MenuGroup, MenuItem, MenuItemIndicator,
    MenuLabel, MenuPortal, MenuRadioGroup, MenuRadioItem, MenuRoot, MenuSeparator, PopperSide,
};

/// Menu Primitive
///
/// The Menu primitive is the internal building block used by DropdownMenu, ContextMenu,
/// and Menubar. It provides:
///
/// RADIX PROVIDES:
/// - Popper-based positioning (side, align, collision detection)
/// - Full keyboard navigation (Arrow keys, Home/End, Enter/Space, Escape)
/// - Typeahead search (type to jump to items)
/// - Focus management (traps focus in modal mode)
/// - Checkbox and Radio items with state management
/// - data-state, data-highlighted, data-disabled attributes
/// - Proper ARIA: role="menu", role="menuitem", etc.
///
/// USER MUST IMPLEMENT:
/// - Trigger for opening the menu (MenuAnchor provides the positioning reference)
/// - Menu styling
/// - Item hover/focus styles (using data-highlighted)
/// - Animations (using data-state)

#[component]
pub fn MenuExample() -> impl IntoView {
    let open = RwSignal::new(false);

    // Checkbox states
    let show_toolbar = RwSignal::new(true);
    let show_sidebar = RwSignal::new(false);

    // Radio state
    let view_mode = RwSignal::new("grid".to_string());

    view! {
        <h1>"Menu (Internal Primitive)"</h1>
        <p>
            "The Menu primitive is the foundation for DropdownMenu, ContextMenu, and Menubar. "
            "It provides positioning via Popper, keyboard navigation, typeahead, and item state management."
        </p>

        <div class="example-section">
            <h2>"Basic Menu"</h2>

            <MenuRoot
                open=open
                on_open_change=Callback::new(move |v| open.set(v))
            >
                <MenuAnchor>
                    <button
                        class="trigger-button"
                        on:click=move |_| open.set(!open.get())
                    >
                        {move || if open.get() { "Close Menu" } else { "Open Menu" }}
                    </button>
                </MenuAnchor>

                <MenuPortal>
                    <MenuContent
                        class="menu-content"
                        side=Signal::derive(|| PopperSide::Bottom)
                        side_offset=Signal::derive(|| 5)
                    >
                        <MenuLabel class="menu-label">"Actions"</MenuLabel>
                        <MenuGroup>
                            <MenuItem class="menu-item" text_value="New File">
                                "New File"
                                <span class="menu-shortcut">"Ctrl+N"</span>
                            </MenuItem>
                            <MenuItem class="menu-item" text_value="New Window">
                                "New Window"
                                <span class="menu-shortcut">"Ctrl+Shift+N"</span>
                            </MenuItem>
                            <MenuItem class="menu-item" text_value="Open" disabled=true>
                                "Open..."
                            </MenuItem>
                        </MenuGroup>

                        <MenuSeparator class="menu-separator" />

                        <MenuLabel class="menu-label">"View"</MenuLabel>
                        <MenuGroup>
                            <MenuCheckboxItem
                                class="menu-checkbox-item"
                                checked=show_toolbar
                                text_value="Show Toolbar"
                            >
                                <MenuItemIndicator class="menu-item-indicator">
                                    "✓"
                                </MenuItemIndicator>
                                "Show Toolbar"
                            </MenuCheckboxItem>
                            <MenuCheckboxItem
                                class="menu-checkbox-item"
                                checked=show_sidebar
                                text_value="Show Sidebar"
                            >
                                <MenuItemIndicator class="menu-item-indicator">
                                    "✓"
                                </MenuItemIndicator>
                                "Show Sidebar"
                            </MenuCheckboxItem>
                        </MenuGroup>

                        <MenuSeparator class="menu-separator" />

                        <MenuLabel class="menu-label">"View Mode"</MenuLabel>
                        <MenuRadioGroup value=view_mode>
                            <MenuRadioItem
                                class="menu-radio-item"
                                value="grid"
                                text_value="Grid View"
                            >
                                <MenuItemIndicator class="menu-item-indicator">
                                    "●"
                                </MenuItemIndicator>
                                "Grid View"
                            </MenuRadioItem>
                            <MenuRadioItem
                                class="menu-radio-item"
                                value="list"
                                text_value="List View"
                            >
                                <MenuItemIndicator class="menu-item-indicator">
                                    "●"
                                </MenuItemIndicator>
                                "List View"
                            </MenuRadioItem>
                            <MenuRadioItem
                                class="menu-radio-item"
                                value="columns"
                                text_value="Column View"
                            >
                                <MenuItemIndicator class="menu-item-indicator">
                                    "●"
                                </MenuItemIndicator>
                                "Column View"
                            </MenuRadioItem>
                        </MenuRadioGroup>

                        <MenuSeparator class="menu-separator" />

                        <MenuItem class="menu-item" text_value="Quit">
                            "Quit"
                            <span class="menu-shortcut">"Ctrl+Q"</span>
                        </MenuItem>

                        <MenuArrow class="menu-arrow" />
                    </MenuContent>
                </MenuPortal>
            </MenuRoot>
        </div>

        <div class="example-section">
            <h2>"Current State"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>"Menu open: " {move || if open.get() { "Yes" } else { "No" }}</li>
                <li>"Show Toolbar: " {move || if show_toolbar.get() { "Yes" } else { "No" }}</li>
                <li>"Show Sidebar: " {move || if show_sidebar.get() { "Yes" } else { "No" }}</li>
                <li>"View Mode: " {move || view_mode.get()}</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"ArrowDown"</code>" - Move to next item"</li>
                <li><code>"ArrowUp"</code>" - Move to previous item"</li>
                <li><code>"Home / PageUp"</code>" - Move to first item"</li>
                <li><code>"End / PageDown"</code>" - Move to last item"</li>
                <li><code>"Enter / Space"</code>" - Select item"</li>
                <li><code>"Escape"</code>" - Close menu"</li>
                <li>"Type letters to jump to matching items (typeahead)"</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"Data Attributes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"data-highlighted"</code>" - Present when item is focused"</li>
                <li><code>"data-disabled"</code>" - Present when item is disabled"</li>
                <li><code>"data-state"</code>" - \"checked\" | \"unchecked\" for checkbox/radio items"</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"Component Structure"</h2>
            <pre style="font-size: 0.75rem; background: #f5f5f5; color: #1a1a1a; padding: 1rem; border-radius: 4px; overflow-x: auto">
{r#"<MenuRoot open={signal} on_open_change={callback}>
  <MenuAnchor>
    <button>Trigger</button>
  </MenuAnchor>
  <MenuPortal>
    <MenuContent side={side} side_offset={offset}>
      <MenuLabel>Section</MenuLabel>
      <MenuGroup>
        <MenuItem>Item</MenuItem>
        <MenuCheckboxItem checked={signal}>
          <MenuItemIndicator>✓</MenuItemIndicator>
          Checkbox
        </MenuCheckboxItem>
      </MenuGroup>
      <MenuSeparator />
      <MenuRadioGroup value={signal}>
        <MenuRadioItem value="a">
          <MenuItemIndicator>●</MenuItemIndicator>
          Option A
        </MenuRadioItem>
      </MenuRadioGroup>
      <MenuArrow />
    </MenuContent>
  </MenuPortal>
</MenuRoot>"#}
            </pre>
        </div>
    }
}
