use leptos::prelude::*;
use radix::ToggleRoot;

/// Toggle Primitive
///
/// RADIX PROVIDES:
/// - On/off state management (controlled or uncontrolled)
/// - data-state="on" | "off"
/// - data-disabled when disabled
/// - aria-pressed attribute
/// - Keyboard activation (Enter/Space)
///
/// USER MUST IMPLEMENT:
/// - Button styling for both states
/// - Visual indicator of pressed state
/// - Disabled styling
///
/// NOTE: This is the simplest Radix primitive - just manages binary state

#[component]
pub fn ToggleExample() -> impl IntoView {
    let bold_pressed = RwSignal::new(false);
    let italic_pressed = RwSignal::new(false);
    let underline_pressed = RwSignal::new(false);

    view! {
        <h1>"Toggle"</h1>
        <p>
            "On/off button state. The simplest Radix primitive - just manages binary "
            "pressed state with proper ARIA. User styles both states."
        </p>

        <div class="example-section">
            <h2>"Basic Toggle"</h2>
            <ToggleRoot class="toggle-button" pressed=RwSignal::new(false) aria_label="Toggle feature">
                "Toggle me"
            </ToggleRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Inspect element to see data-state change between \"on\" and \"off\""
            </p>
        </div>

        <div class="example-section">
            <h2>"Default Pressed"</h2>
            <ToggleRoot class="toggle-button" pressed=RwSignal::new(true) aria_label="Toggle feature">
                "On by default"
            </ToggleRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled State"</h2>
            <div style="display: flex; gap: 0.5rem; align-items: center">
                <ToggleRoot class="toggle-button" pressed=bold_pressed aria_label="Toggle bold">
                    <strong>"B"</strong>
                </ToggleRoot>
                <ToggleRoot class="toggle-button" pressed=italic_pressed aria_label="Toggle italic">
                    <em>"I"</em>
                </ToggleRoot>
                <ToggleRoot class="toggle-button" pressed=underline_pressed aria_label="Toggle underline">
                    <span style="text-decoration: underline">"U"</span>
                </ToggleRoot>
            </div>
            <p
                style:margin-top="1rem"
                style:font-weight=move || if bold_pressed.get() { "bold" } else { "normal" }
                style:font-style=move || if italic_pressed.get() { "italic" } else { "normal" }
                style:text-decoration=move || if underline_pressed.get() { "underline" } else { "none" }
            >
                "Sample text with formatting"
            </p>
            <p style="font-size: 0.875rem; margin-top: 0.5rem">
                "State: Bold=" {move || bold_pressed.get().to_string()}
                ", Italic=" {move || italic_pressed.get().to_string()}
                ", Underline=" {move || underline_pressed.get().to_string()}
            </p>
        </div>

        <div class="example-section">
            <h2>"Disabled Toggle"</h2>
            <div style="display: flex; gap: 0.5rem">
                <ToggleRoot class="toggle-button" pressed=RwSignal::new(false) disabled=true aria_label="Disabled off">
                    "Disabled (off)"
                </ToggleRoot>
                <ToggleRoot class="toggle-button" pressed=RwSignal::new(true) disabled=true aria_label="Disabled on">
                    "Disabled (on)"
                </ToggleRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"Icon Toggles"</h2>
            <div style="display: flex; gap: 0.5rem">
                <ToggleRoot
                    class="toggle-button"
                    pressed=RwSignal::new(false)
                    aria_label="Toggle dark mode"
                    style="font-size: 1.5rem; padding: 0.5rem"
                >
                    "\u{1F319}"
                </ToggleRoot>
                <ToggleRoot
                    class="toggle-button"
                    pressed=RwSignal::new(false)
                    aria_label="Toggle sound"
                    style="font-size: 1.5rem; padding: 0.5rem"
                >
                    "\u{1F50A}"
                </ToggleRoot>
                <ToggleRoot
                    class="toggle-button"
                    pressed=RwSignal::new(false)
                    aria_label="Toggle favorite"
                    style="font-size: 1.5rem; padding: 0.5rem"
                >
                    "\u{2B50}"
                </ToggleRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"What Radix Toggle Provides"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"data-state=\"on\" | \"off\""</code>" - for CSS styling"
                </li>
                <li>
                    <code>"aria-pressed"</code>" - accessibility for screen readers"
                </li>
                <li>
                    <code>"pressed"</code>" - controlled mode via RwSignal"
                </li>
                <li>
                    <code>"disabled"</code>" - disables interaction"
                </li>
                <li>"Keyboard support: Enter and Space toggle the button"</li>
            </ul>
        </div>
    }
}
