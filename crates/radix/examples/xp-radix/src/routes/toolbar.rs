use leptos::prelude::*;
use leptos::web_sys;
use radix::{
    ToolbarButton, ToolbarLink, ToolbarOrientation, ToolbarRoot, ToolbarSeparator,
    ToolbarToggleGroup, ToolbarToggleItem, ToolbarToggleType,
};

/// Toolbar Primitive
///
/// RADIX PROVIDES:
/// - role="toolbar" with keyboard navigation
/// - Arrow key navigation between items
/// - Roving tabindex (Tab into toolbar, arrows within)
/// - ToggleGroup integration
/// - Separator for grouping
/// - Button, Link, and ToggleItem parts
/// - data-orientation="horizontal" | "vertical"
///
/// USER MUST IMPLEMENT:
/// - Toolbar container styling
/// - Button/item styling
/// - Separator styling
/// - Toggle states

#[component]
pub fn ToolbarExample() -> impl IntoView {
    view! {
        <h1>"Toolbar"</h1>
        <p>
            "Toolbar container with buttons, links, and toggle groups. Radix handles "
            "keyboard navigation and ARIA. User styles all visual elements."
        </p>

        <div class="example-section">
            <h2>"Basic Toolbar"</h2>
            <ToolbarRoot class="toolbar-root" aria_label="Formatting options">
                <ToolbarButton
                    class="toolbar-button"
                    on_click=Callback::new(|_| {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Cut")
                            .unwrap();
                    })
                >
                    "Cut"
                </ToolbarButton>
                <ToolbarButton
                    class="toolbar-button"
                    on_click=Callback::new(|_| {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Copy")
                            .unwrap();
                    })
                >
                    "Copy"
                </ToolbarButton>
                <ToolbarButton
                    class="toolbar-button"
                    on_click=Callback::new(|_| {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Paste")
                            .unwrap();
                    })
                >
                    "Paste"
                </ToolbarButton>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarButton
                    class="toolbar-button"
                    on_click=Callback::new(|_| {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Delete")
                            .unwrap();
                    })
                >
                    "Delete"
                </ToolbarButton>
            </ToolbarRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys to navigate, Tab to exit toolbar"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Toggle Group"</h2>
            <ToolbarRoot class="toolbar-root" aria_label="Text formatting">
                <ToolbarToggleGroup toggle_type=ToolbarToggleType::Multiple aria_label="Text style">
                    <ToolbarToggleItem class="toolbar-toggle-item" value="bold" aria_label="Bold">
                        <strong>"B"</strong>
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="italic" aria_label="Italic">
                        <em>"I"</em>
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="underline" aria_label="Underline">
                        <span style="text-decoration: underline">"U"</span>
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarToggleGroup
                    toggle_type=ToolbarToggleType::Single
                    default_value="left"
                    aria_label="Text alignment"
                >
                    <ToolbarToggleItem class="toolbar-toggle-item" value="left" aria_label="Left align">
                        "\u{2190}"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="center" aria_label="Center align">
                        "\u{2194}"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="right" aria_label="Right align">
                        "\u{2192}"
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
            </ToolbarRoot>
        </div>

        <div class="example-section">
            <h2>"With Links"</h2>
            <ToolbarRoot class="toolbar-root" aria_label="Navigation">
                <ToolbarLink
                    class="toolbar-link"
                    href="#home"
                    on_click=Callback::new(|ev: web_sys::MouseEvent| {
                        ev.prevent_default();
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Home clicked")
                            .unwrap();
                    })
                >
                    "Home"
                </ToolbarLink>
                <ToolbarLink
                    class="toolbar-link"
                    href="#about"
                    on_click=Callback::new(|ev: web_sys::MouseEvent| {
                        ev.prevent_default();
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("About clicked")
                            .unwrap();
                    })
                >
                    "About"
                </ToolbarLink>
                <ToolbarLink
                    class="toolbar-link"
                    href="#contact"
                    on_click=Callback::new(|ev: web_sys::MouseEvent| {
                        ev.prevent_default();
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Contact clicked")
                            .unwrap();
                    })
                >
                    "Contact"
                </ToolbarLink>
            </ToolbarRoot>
        </div>

        <div class="example-section">
            <h2>"Editor Toolbar"</h2>
            <ToolbarRoot class="toolbar-root" aria_label="Editor tools">
                <ToolbarButton class="toolbar-button">"New"</ToolbarButton>
                <ToolbarButton class="toolbar-button">"Open"</ToolbarButton>
                <ToolbarButton class="toolbar-button">"Save"</ToolbarButton>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarToggleGroup toggle_type=ToolbarToggleType::Multiple aria_label="Format">
                    <ToolbarToggleItem class="toolbar-toggle-item" value="bold" aria_label="Bold">
                        "B"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="italic" aria_label="Italic">
                        "I"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="code" aria_label="Code">
                        "</>"
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarButton class="toolbar-button">"Undo"</ToolbarButton>
                <ToolbarButton class="toolbar-button">"Redo"</ToolbarButton>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarLink class="toolbar-link" href="https://example.com" target="_blank">
                    "Help"
                </ToolbarLink>
            </ToolbarRoot>
        </div>

        <div class="example-section">
            <h2>"Vertical Toolbar"</h2>
            <ToolbarRoot
                class="toolbar-root"
                orientation=ToolbarOrientation::Vertical
                aria_label="Drawing tools"
                style="flex-direction: column; width: fit-content"
            >
                <ToolbarToggleGroup
                    toggle_type=ToolbarToggleType::Single
                    default_value="select"
                    aria_label="Tool"
                >
                    <ToolbarToggleItem class="toolbar-toggle-item" value="select" aria_label="Select">
                        "\u{2196}"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="pen" aria_label="Pen">
                        "\u{270F}"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="eraser" aria_label="Eraser">
                        "\u{232B}"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="fill" aria_label="Fill">
                        "\u{25C9}"
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
                <ToolbarSeparator class="toolbar-separator" style="height: 1px; width: 100%" />
                <ToolbarButton class="toolbar-button">"Clear"</ToolbarButton>
            </ToolbarRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Up/Down arrows navigate vertical toolbars"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Disabled Items"</h2>
            <ToolbarRoot class="toolbar-root" aria_label="Edit actions">
                <ToolbarButton class="toolbar-button">"Edit"</ToolbarButton>
                <ToolbarButton class="toolbar-button" disabled=true>
                    "Save (disabled)"
                </ToolbarButton>
                <ToolbarSeparator class="toolbar-separator" />
                <ToolbarToggleGroup toggle_type=ToolbarToggleType::Single aria_label="Mode">
                    <ToolbarToggleItem class="toolbar-toggle-item" value="view">
                        "View"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem class="toolbar-toggle-item" value="edit" disabled=true>
                        "Edit"
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
            </ToolbarRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Arrow keys skip disabled items"
            </p>
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"Tab"</code>" - Enter/exit toolbar"</li>
                <li><code>"Arrow keys"</code>" - Navigate between items"</li>
                <li><code>"Home/End"</code>" - Jump to first/last item"</li>
                <li><code>"Enter/Space"</code>" - Activate button or toggle"</li>
            </ul>
        </div>
    }
}
