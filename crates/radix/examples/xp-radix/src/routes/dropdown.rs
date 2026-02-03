use leptos::prelude::*;
use radix::{
    DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuPortal, DropdownMenuRoot, DropdownMenuSeparator, DropdownMenuTrigger,
};

/// DropdownMenu Primitive
///
/// RADIX PROVIDES:
/// - Positioning relative to trigger (with collision detection)
/// - Keyboard navigation (arrow keys)
/// - Focus management (roving tabindex within menu)
/// - data-highlighted on focused item
/// - data-state="open" | "closed"
/// - data-disabled on disabled items
/// - Proper ARIA: role="menu", role="menuitem", etc.
///
/// USER MUST IMPLEMENT:
/// - Menu container styling
/// - Item hover/focus states (use data-highlighted)
/// - Separator and label styling

#[component]
pub fn DropdownExample() -> impl IntoView {
    view! {
        <h1>"DropdownMenu"</h1>
        <p>
            "Menu triggered by button. Radix handles positioning, keyboard "
            "navigation, and ARIA. User styles all visual elements."
        </p>

        <div class="example-section">
            <h2>"Basic Menu"</h2>
            <DropdownMenuRoot>
                <DropdownMenuTrigger>
                    "Options"
                </DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class="dropdown-content" side_offset=5>
                        <DropdownMenuItem class="dropdown-item">
                            "New Tab"
                        </DropdownMenuItem>
                        <DropdownMenuItem class="dropdown-item">
                            "New Window"
                        </DropdownMenuItem>
                        <DropdownMenuItem class="dropdown-item" disabled=true>
                            "New Private Window"
                        </DropdownMenuItem>
                        <DropdownMenuSeparator class="dropdown-separator" />
                        <DropdownMenuItem class="dropdown-item">
                            "Settings"
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenuRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys to navigate, Enter to select, Escape to close"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Labels and Groups"</h2>
            <DropdownMenuRoot>
                <DropdownMenuTrigger>
                    "Account"
                </DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class="dropdown-content" side_offset=5>
                        <DropdownMenuLabel class="dropdown-label">
                            "Signed in as john@example.com"
                        </DropdownMenuLabel>
                        <DropdownMenuSeparator class="dropdown-separator" />
                        <DropdownMenuGroup>
                            <DropdownMenuItem class="dropdown-item">
                                "Profile"
                            </DropdownMenuItem>
                            <DropdownMenuItem class="dropdown-item">
                                "Settings"
                            </DropdownMenuItem>
                            <DropdownMenuItem class="dropdown-item">
                                "Billing"
                            </DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator class="dropdown-separator" />
                        <DropdownMenuItem class="dropdown-item">
                            "Sign out"
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenuRoot>
        </div>

        <div class="example-section">
            <h2>"With Actions"</h2>
            <DropdownMenuRoot>
                <DropdownMenuTrigger>
                    "Actions"
                </DropdownMenuTrigger>
                <DropdownMenuPortal>
                    <DropdownMenuContent class="dropdown-content" side_offset=5>
                        <DropdownMenuItem class="dropdown-item">
                            "Undo"
                        </DropdownMenuItem>
                        <DropdownMenuItem class="dropdown-item">
                            "Redo"
                        </DropdownMenuItem>
                        <DropdownMenuSeparator class="dropdown-separator" />
                        <DropdownMenuItem class="dropdown-item">
                            "Cut"
                        </DropdownMenuItem>
                        <DropdownMenuItem class="dropdown-item">
                            "Copy"
                        </DropdownMenuItem>
                        <DropdownMenuItem class="dropdown-item">
                            "Paste"
                        </DropdownMenuItem>
                        <DropdownMenuSeparator class="dropdown-separator" />
                        <DropdownMenuItem class="dropdown-item">
                            "Delete"
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenuPortal>
            </DropdownMenuRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Right arrow to open submenu, Left arrow to close"
            </p>
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"ArrowDown"</code>" / "<code>"ArrowUp"</code>" - Navigate items"</li>
                <li><code>"Enter"</code>" / "<code>"Space"</code>" - Select item"</li>
                <li><code>"Escape"</code>" - Close menu (focus returns to trigger)"</li>
                <li><code>"Home"</code>" / "<code>"End"</code>" - Jump to first/last item"</li>
            </ul>
        </div>
    }
}
