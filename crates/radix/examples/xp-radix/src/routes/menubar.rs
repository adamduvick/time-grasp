use leptos::prelude::*;
use radix::{
    MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarItemIndicator, MenubarMenu,
    MenubarPortal, MenubarRadioGroup, MenubarRadioItem, MenubarRoot, MenubarSeparator,
    MenubarTrigger,
};

/// Menubar Primitive
///
/// RADIX PROVIDES:
/// - Application menu bar pattern
/// - Keyboard navigation (arrows between menus)
/// - Auto-open adjacent menus when one is open
/// - Same features as DropdownMenu within each menu:
///   - Submenus, checkbox/radio items, separators
///   - data-highlighted, data-disabled
/// - Proper ARIA: role="menubar", role="menuitem"
///
/// USER MUST IMPLEMENT:
/// - Menubar container styling
/// - Trigger styling
/// - Menu content styling (same as DropdownMenu)

#[component]
pub fn MenubarExample() -> impl IntoView {
    // State for checkbox items
    let show_toolbar = RwSignal::new(true);
    let show_sidebar = RwSignal::new(false);
    let show_statusbar = RwSignal::new(true);

    // State for radio group
    let profile = RwSignal::new("personal".to_string());

    view! {
        <h1>"Menubar"</h1>
        <p>
            "Application menu bar with keyboard navigation. When one menu is open, "
            "arrow keys move to adjacent menus. Uses same patterns as DropdownMenu."
        </p>

        <div class="example-section">
            <h2>"Basic Menubar"</h2>

            <MenubarRoot class="menubar-root">
                <MenubarMenu>
                    <MenubarTrigger class="menubar-trigger">"File"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent class="menubar-content" side_offset=5>
                            <MenubarItem class="menubar-item" text_value="New Tab">
                                "New Tab "
                                <span class="menubar-shortcut">"⌘T"</span>
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="New Window">
                                "New Window "
                                <span class="menubar-shortcut">"⌘N"</span>
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="New Incognito Window" disabled=true>
                                "New Incognito Window"
                            </MenubarItem>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Share">
                                "Share"
                            </MenubarItem>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Print">
                                "Print... "
                                <span class="menubar-shortcut">"⌘P"</span>
                            </MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>

                <MenubarMenu>
                    <MenubarTrigger class="menubar-trigger">"Edit"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent class="menubar-content" side_offset=5>
                            <MenubarItem class="menubar-item" text_value="Undo">
                                "Undo "
                                <span class="menubar-shortcut">"⌘Z"</span>
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="Redo">
                                "Redo "
                                <span class="menubar-shortcut">"⇧⌘Z"</span>
                            </MenubarItem>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Cut">
                                "Cut "
                                <span class="menubar-shortcut">"⌘X"</span>
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="Copy">
                                "Copy "
                                <span class="menubar-shortcut">"⌘C"</span>
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="Paste">
                                "Paste "
                                <span class="menubar-shortcut">"⌘V"</span>
                            </MenubarItem>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Select All">
                                "Select All "
                                <span class="menubar-shortcut">"⌘A"</span>
                            </MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>

                <MenubarMenu>
                    <MenubarTrigger class="menubar-trigger">"View"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent class="menubar-content" side_offset=5>
                            <MenubarCheckboxItem
                                class="menubar-item"
                                checked=show_toolbar
                                text_value="Show Toolbar"
                            >
                                <MenubarItemIndicator>"✓ "</MenubarItemIndicator>
                                "Show Toolbar"
                            </MenubarCheckboxItem>
                            <MenubarCheckboxItem
                                class="menubar-item"
                                checked=show_sidebar
                                text_value="Show Sidebar"
                            >
                                <MenubarItemIndicator>"✓ "</MenubarItemIndicator>
                                "Show Sidebar"
                            </MenubarCheckboxItem>
                            <MenubarCheckboxItem
                                class="menubar-item"
                                checked=show_statusbar
                                text_value="Show Status Bar"
                            >
                                <MenubarItemIndicator>"✓ "</MenubarItemIndicator>
                                "Show Status Bar"
                            </MenubarCheckboxItem>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Enter Full Screen">
                                "Enter Full Screen "
                                <span class="menubar-shortcut">"^⌘F"</span>
                            </MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>

                <MenubarMenu>
                    <MenubarTrigger class="menubar-trigger">"Profiles"</MenubarTrigger>
                    <MenubarPortal>
                        <MenubarContent class="menubar-content" side_offset=5>
                            <MenubarRadioGroup value=profile>
                                <MenubarRadioItem class="menubar-item" value="personal" text_value="Personal">
                                    <MenubarItemIndicator>"• "</MenubarItemIndicator>
                                    "Personal"
                                </MenubarRadioItem>
                                <MenubarRadioItem class="menubar-item" value="work" text_value="Work">
                                    <MenubarItemIndicator>"• "</MenubarItemIndicator>
                                    "Work"
                                </MenubarRadioItem>
                                <MenubarRadioItem class="menubar-item" value="school" text_value="School">
                                    <MenubarItemIndicator>"• "</MenubarItemIndicator>
                                    "School"
                                </MenubarRadioItem>
                            </MenubarRadioGroup>
                            <MenubarSeparator class="menubar-separator" />
                            <MenubarItem class="menubar-item" text_value="Add Profile">
                                "Add Profile..."
                            </MenubarItem>
                            <MenubarItem class="menubar-item" text_value="Edit Profiles">
                                "Edit Profiles..."
                            </MenubarItem>
                        </MenubarContent>
                    </MenubarPortal>
                </MenubarMenu>
            </MenubarRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Open a menu, then use Left/Right arrows to move between menus"
            </p>
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"Enter/Space"</code>" - Open menu"</li>
                <li><code>"Arrow Down"</code>" - Open menu / move to next item"</li>
                <li><code>"Arrow Up"</code>" - Move to previous item"</li>
                <li><code>"Arrow Left/Right"</code>" - Move between menus (when open)"</li>
                <li><code>"Esc"</code>" - Close menu"</li>
                <li>"Typing jumps to matching items"</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"Menubar vs DropdownMenu"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Menubar"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"DropdownMenu"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Multiple menus"</td>
                        <td style="padding: 0.5rem">"Yes, in a bar"</td>
                        <td style="padding: 0.5rem">"Single menu"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Auto-open adjacent"</td>
                        <td style="padding: 0.5rem">"Yes"</td>
                        <td style="padding: 0.5rem">"N/A"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Left/Right navigation"</td>
                        <td style="padding: 0.5rem">"Between menus"</td>
                        <td style="padding: 0.5rem">"Submenus only"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"ARIA role"</td>
                        <td style="padding: 0.5rem">"menubar"</td>
                        <td style="padding: 0.5rem">"menu"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
