use leptos::prelude::*;
use radix::{CheckboxIndicator, CheckboxRoot, CheckedState};

/// Checkbox Primitive
///
/// RADIX PROVIDES:
/// - Tri-state: checked, unchecked, indeterminate
/// - data-state="checked" | "unchecked" | "indeterminate"
/// - data-disabled when disabled
/// - aria-checked with "true", "false", or "mixed"
/// - Indicator component (only renders when checked/indeterminate)
/// - Controlled and uncontrolled modes
///
/// USER MUST IMPLEMENT:
/// - Checkbox box styling
/// - Checkmark/indicator styling
/// - Label association (use htmlFor or wrap)
/// - Focus ring styling

#[component]
pub fn CheckboxExample() -> impl IntoView {
    // For the indeterminate demo
    let indeterminate_demo = RwSignal::new(CheckedState::Indeterminate);

    // For the select-all demo
    let checked1 = RwSignal::new(CheckedState::Checked);
    let checked2 = RwSignal::new(CheckedState::Unchecked);
    let checked3 = RwSignal::new(CheckedState::Unchecked);

    // Compute parent state based on children
    let parent_state = Signal::derive(move || {
        let c1 = checked1.get() == CheckedState::Checked;
        let c2 = checked2.get() == CheckedState::Checked;
        let c3 = checked3.get() == CheckedState::Checked;

        if c1 && c2 && c3 {
            CheckedState::Checked
        } else if c1 || c2 || c3 {
            CheckedState::Indeterminate
        } else {
            CheckedState::Unchecked
        }
    });

    let parent_checked = RwSignal::new(CheckedState::Checked);

    // Keep parent_checked in sync with computed state
    Effect::new(move |_| {
        parent_checked.set(parent_state.get());
    });

    let handle_parent_change = move |_| {
        let new_value = if parent_state.get() != CheckedState::Checked {
            CheckedState::Checked
        } else {
            CheckedState::Unchecked
        };
        checked1.set(new_value);
        checked2.set(new_value);
        checked3.set(new_value);
    };

    view! {
        <h1>"Checkbox"</h1>
        <p>
            "Tri-state checkbox. Radix handles checked/unchecked/indeterminate "
            "states, ARIA, and keyboard. User styles the box and indicator."
        </p>

        <div class="example-section">
            <h2>"Basic Checkbox"</h2>
            <div style="display: flex; align-items: center; gap: 0.5rem">
                <CheckboxRoot class="checkbox-root" id="c1" checked=RwSignal::new(CheckedState::Checked)>
                    <CheckboxIndicator class="checkbox-indicator">
                        "\u{2713}"
                    </CheckboxIndicator>
                </CheckboxRoot>
                <label for="c1">"Accept terms and conditions"</label>
            </div>
        </div>

        <div class="example-section">
            <h2>"Checkbox States"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.75rem">
                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="unchecked" checked=RwSignal::new(CheckedState::Unchecked)>
                        <CheckboxIndicator class="checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="unchecked">"Unchecked"</label>
                </div>

                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="checked" checked=RwSignal::new(CheckedState::Checked)>
                        <CheckboxIndicator class="checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="checked">"Checked"</label>
                </div>

                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="indeterminate" checked=indeterminate_demo>
                        <CheckboxIndicator class="checkbox-indicator">
                            {move || if indeterminate_demo.get() == CheckedState::Indeterminate { "\u{2212}" } else { "\u{2713}" }}
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="indeterminate">"Indeterminate (click to toggle)"</label>
                </div>

                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="disabled" checked=RwSignal::new(CheckedState::Unchecked) disabled=true>
                        <CheckboxIndicator class="checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="disabled" style="color: var(--color-text-muted)">"Disabled"</label>
                </div>

                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="disabled-checked" checked=RwSignal::new(CheckedState::Checked) disabled=true>
                        <CheckboxIndicator class="checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="disabled-checked" style="color: var(--color-text-muted)">"Disabled (checked)"</label>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Indeterminate Parent (Select All)"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.75rem">
                <div style="display: flex; align-items: center; gap: 0.5rem">
                    <CheckboxRoot class="checkbox-root" id="parent" checked=parent_checked on:click=handle_parent_change>
                        <CheckboxIndicator class="checkbox-indicator">
                            {move || if parent_state.get() == CheckedState::Indeterminate { "\u{2212}" } else { "\u{2713}" }}
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="parent" style="font-weight: bold">"Select all"</label>
                </div>

                <div style="margin-left: 1.5rem; display: flex; flex-direction: column; gap: 0.5rem">
                    <div style="display: flex; align-items: center; gap: 0.5rem">
                        <CheckboxRoot class="checkbox-root" id="child1" checked=checked1>
                            <CheckboxIndicator class="checkbox-indicator">
                                "\u{2713}"
                            </CheckboxIndicator>
                        </CheckboxRoot>
                        <label for="child1">"Option 1"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem">
                        <CheckboxRoot class="checkbox-root" id="child2" checked=checked2>
                            <CheckboxIndicator class="checkbox-indicator">
                                "\u{2713}"
                            </CheckboxIndicator>
                        </CheckboxRoot>
                        <label for="child2">"Option 2"</label>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.5rem">
                        <CheckboxRoot class="checkbox-root" id="child3" checked=checked3>
                            <CheckboxIndicator class="checkbox-indicator">
                                "\u{2713}"
                            </CheckboxIndicator>
                        </CheckboxRoot>
                        <label for="child3">"Option 3"</label>
                    </div>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Form Usage"</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
                // Form handling would go here
            }>
                <div style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 1rem">
                    <CheckboxRoot
                        class="checkbox-root"
                        id="newsletter"
                        checked=RwSignal::new(CheckedState::Unchecked)
                        name="newsletter"
                        value="yes"
                    >
                        <CheckboxIndicator class="checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </CheckboxRoot>
                    <label for="newsletter">"Subscribe to newsletter"</label>
                </div>
                <button type="submit" class="trigger-button">
                    "Submit"
                </button>
            </form>
        </div>
    }
}
