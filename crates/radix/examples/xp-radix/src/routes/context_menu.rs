use leptos::prelude::*;
use radix::{
    ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuItemIndicator,
    ContextMenuLabel, ContextMenuPortal, ContextMenuRadioGroup, ContextMenuRadioItem,
    ContextMenuRoot, ContextMenuSeparator, ContextMenuSub, ContextMenuSubContent,
    ContextMenuSubTrigger, ContextMenuTrigger,
};

/// ContextMenu Primitive
///
/// RADIX PROVIDES:
/// - Right-click trigger (or long-press on touch)
/// - Same features as DropdownMenu:
///   - Keyboard navigation
///   - Submenus
///   - Checkbox/Radio items
///   - data-highlighted, data-disabled
/// - Positioning at pointer location
/// - Proper ARIA roles
///
/// USER MUST IMPLEMENT:
/// - Trigger area styling (where right-click is active)
/// - Menu styling (same as DropdownMenu)
///
/// NOTE: Shares most parts with DropdownMenu (Item, Sub, Separator, etc.)

#[component]
pub fn ContextMenuExample() -> impl IntoView {
    // State for checkbox items
    let show_hidden_files = RwSignal::new(true);
    let show_extensions = RwSignal::new(false);

    // State for radio group
    let sort_by = RwSignal::new("name".to_string());

    view! {
        <h1>"ContextMenu"</h1>
        <p>
            "Right-click menu. Same features as DropdownMenu but triggered by "
            "right-click instead of button click. Positioning follows the pointer."
        </p>

        <div class="example-section">
            <h2>"Basic Context Menu"</h2>
            <ContextMenuRoot>
                <ContextMenuTrigger class="context-trigger">
                    "Right-click here"
                </ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent class="dropdown-content">
                        <ContextMenuItem class="dropdown-item">
                            "Cut"
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item">
                            "Copy"
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item">
                            "Paste"
                        </ContextMenuItem>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuItem class="dropdown-item" disabled=true>
                            "Paste Special..."
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenuRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "On touch devices, long-press triggers the menu"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Submenus"</h2>
            <ContextMenuRoot>
                <ContextMenuTrigger class="context-trigger">
                    "Right-click for file options"
                </ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent class="dropdown-content">
                        <ContextMenuItem class="dropdown-item">
                            "New File"
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item">
                            "New Folder"
                        </ContextMenuItem>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuSub>
                            <ContextMenuSubTrigger class="dropdown-item dropdown-sub-trigger">
                                "Open With"
                                <span>"→"</span>
                            </ContextMenuSubTrigger>
                            <ContextMenuSubContent class="dropdown-sub-content">
                                <ContextMenuItem class="dropdown-item">
                                    "VS Code"
                                </ContextMenuItem>
                                <ContextMenuItem class="dropdown-item">
                                    "Sublime Text"
                                </ContextMenuItem>
                                <ContextMenuItem class="dropdown-item">
                                    "Notepad"
                                </ContextMenuItem>
                                <ContextMenuSeparator class="dropdown-separator" />
                                <ContextMenuItem class="dropdown-item">
                                    "Choose another app..."
                                </ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                        <ContextMenuSub>
                            <ContextMenuSubTrigger class="dropdown-item dropdown-sub-trigger">
                                "Share"
                                <span>"→"</span>
                            </ContextMenuSubTrigger>
                            <ContextMenuSubContent class="dropdown-sub-content">
                                <ContextMenuItem class="dropdown-item">
                                    "Email"
                                </ContextMenuItem>
                                <ContextMenuItem class="dropdown-item">
                                    "AirDrop"
                                </ContextMenuItem>
                                <ContextMenuItem class="dropdown-item">
                                    "Copy Link"
                                </ContextMenuItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuItem class="dropdown-item">
                            "Rename"
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item" style="color: #dc2626">
                            "Delete"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenuRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Right arrow to open submenu, Left arrow to close"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Checkboxes and Radio"</h2>
            <ContextMenuRoot>
                <ContextMenuTrigger class="context-trigger">
                    "Right-click for view options"
                </ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent class="dropdown-content">
                        <ContextMenuLabel class="dropdown-label">
                            "View"
                        </ContextMenuLabel>
                        <ContextMenuCheckboxItem
                            class="dropdown-item"
                            checked=show_hidden_files
                            on_checked_change=Callback::new(move |v| show_hidden_files.set(v))
                        >
                            <ContextMenuItemIndicator>"✓ "</ContextMenuItemIndicator>
                            "Show Hidden Files"
                        </ContextMenuCheckboxItem>
                        <ContextMenuCheckboxItem
                            class="dropdown-item"
                            checked=show_extensions
                            on_checked_change=Callback::new(move |v| show_extensions.set(v))
                        >
                            <ContextMenuItemIndicator>"✓ "</ContextMenuItemIndicator>
                            "Show Extensions"
                        </ContextMenuCheckboxItem>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuLabel class="dropdown-label">
                            "Sort By"
                        </ContextMenuLabel>
                        <ContextMenuRadioGroup
                            value=sort_by
                            on_value_change=Callback::new(move |v| sort_by.set(v))
                        >
                            <ContextMenuRadioItem class="dropdown-item" value="name">
                                <ContextMenuItemIndicator>"• "</ContextMenuItemIndicator>
                                "Name"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem class="dropdown-item" value="date">
                                <ContextMenuItemIndicator>"• "</ContextMenuItemIndicator>
                                "Date Modified"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem class="dropdown-item" value="size">
                                <ContextMenuItemIndicator>"• "</ContextMenuItemIndicator>
                                "Size"
                            </ContextMenuRadioItem>
                            <ContextMenuRadioItem class="dropdown-item" value="kind">
                                <ContextMenuItemIndicator>"• "</ContextMenuItemIndicator>
                                "Kind"
                            </ContextMenuRadioItem>
                        </ContextMenuRadioGroup>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenuRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Current: Hidden Files: "
                {move || if show_hidden_files.get() { "ON" } else { "OFF" }}
                ", Extensions: "
                {move || if show_extensions.get() { "ON" } else { "OFF" }}
                ", Sort: "
                {move || sort_by.get()}
            </p>
        </div>

        <div class="example-section">
            <h2>"Image Context Menu"</h2>
            <ContextMenuRoot>
                <ContextMenuTrigger>
                    <div style="width: 200px; height: 150px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: var(--radius); display: flex; align-items: center; justify-content: center; color: white; cursor: context-menu">
                        "Right-click this image"
                    </div>
                </ContextMenuTrigger>
                <ContextMenuPortal>
                    <ContextMenuContent class="dropdown-content">
                        <ContextMenuItem class="dropdown-item">
                            "Save Image As..."
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item">
                            "Copy Image"
                        </ContextMenuItem>
                        <ContextMenuItem class="dropdown-item">
                            "Copy Image Address"
                        </ContextMenuItem>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuItem class="dropdown-item">
                            "Open Image in New Tab"
                        </ContextMenuItem>
                        <ContextMenuSeparator class="dropdown-separator" />
                        <ContextMenuItem class="dropdown-item">
                            "Inspect Element"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenuPortal>
            </ContextMenuRoot>
        </div>
    }
}
