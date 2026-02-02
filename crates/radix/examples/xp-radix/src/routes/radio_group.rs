use leptos::prelude::*;
use radix::{RadioGroupIndicator, RadioGroupItem, RadioGroupOrientation, RadioGroupRoot};

/// RadioGroup Primitive
///
/// RADIX PROVIDES:
/// - Single selection enforcement
/// - Keyboard navigation (arrow keys)
/// - data-state="checked" | "unchecked" on items
/// - data-disabled when disabled
/// - Proper ARIA: role="radiogroup", role="radio"
/// - Roving tabindex (Tab into group, arrows within)
/// - Indicator component (only renders when checked)
///
/// USER MUST IMPLEMENT:
/// - Radio button styling
/// - Indicator (dot) styling
/// - Label association
/// - Focus ring styling

#[component]
pub fn RadioGroupExample() -> impl IntoView {
    let plan = RwSignal::new("startup".to_string());

    view! {
        <h1>"RadioGroup"</h1>
        <p>
            "Single selection from multiple options. Radix handles selection, keyboard "
            "navigation, and ARIA. User styles the radio buttons."
        </p>

        <div class="example-section">
            <h2>"Basic Radio Group"</h2>
            <RadioGroupRoot
                class="radiogroup-root"
                value=RwSignal::new("comfortable".to_string())
                aria_label="View density"
            >
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="default" id="r1">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="r1">"Default"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="comfortable" id="r2">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="r2">"Comfortable"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="compact" id="r3">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="r3">"Compact"</label>
                </div>
            </RadioGroupRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys to navigate, Tab to move focus in/out of group"
            </p>
        </div>

        <div class="example-section">
            <h2>"Horizontal Layout"</h2>
            <RadioGroupRoot
                class="radiogroup-root"
                value=RwSignal::new("light".to_string())
                orientation=RadioGroupOrientation::Horizontal
                style="flex-direction: row; gap: 1.5rem"
            >
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="light" id="h1">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="h1">"Light"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="dark" id="h2">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="h2">"Dark"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="system" id="h3">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="h3">"System"</label>
                </div>
            </RadioGroupRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled State"</h2>
            <RadioGroupRoot class="radiogroup-root" value=plan>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="startup" id="p1">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="p1">
                        <strong>"Startup"</strong>
                        <span style="display: block; font-size: 0.875rem; color: var(--color-text-muted)">
                            "$29/month - Up to 5 users"
                        </span>
                    </label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="business" id="p2">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="p2">
                        <strong>"Business"</strong>
                        <span style="display: block; font-size: 0.875rem; color: var(--color-text-muted)">
                            "$99/month - Up to 25 users"
                        </span>
                    </label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="enterprise" id="p3">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="p3">
                        <strong>"Enterprise"</strong>
                        <span style="display: block; font-size: 0.875rem; color: var(--color-text-muted)">
                            "Custom pricing - Unlimited users"
                        </span>
                    </label>
                </div>
            </RadioGroupRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Selected plan: " <strong>{move || plan.get()}</strong>
            </p>
        </div>

        <div class="example-section">
            <h2>"With Disabled Options"</h2>
            <RadioGroupRoot class="radiogroup-root" value=RwSignal::new("available".to_string())>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="available" id="d1">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="d1">"Available"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="sold-out" id="d2" disabled=true>
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="d2" style="color: var(--color-text-muted)">"Sold Out (disabled)"</label>
                </div>
                <div class="radiogroup-item-wrapper">
                    <RadioGroupItem class="radiogroup-item" value="also-available" id="d3">
                        <RadioGroupIndicator class="radiogroup-indicator" />
                    </RadioGroupItem>
                    <label for="d3">"Also Available"</label>
                </div>
            </RadioGroupRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Arrow keys skip disabled options"
            </p>
        </div>

        <div class="example-section">
            <h2>"Form Usage"</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
                // Form handling would go here
            }>
                <RadioGroupRoot
                    class="radiogroup-root"
                    value=RwSignal::new("good".to_string())
                    name="rating"
                    style="flex-direction: row; gap: 1rem"
                >
                    <div class="radiogroup-item-wrapper">
                        <RadioGroupItem class="radiogroup-item" value="poor" id="rating-poor">
                            <RadioGroupIndicator class="radiogroup-indicator" />
                        </RadioGroupItem>
                        <label for="rating-poor">"Poor"</label>
                    </div>
                    <div class="radiogroup-item-wrapper">
                        <RadioGroupItem class="radiogroup-item" value="fair" id="rating-fair">
                            <RadioGroupIndicator class="radiogroup-indicator" />
                        </RadioGroupItem>
                        <label for="rating-fair">"Fair"</label>
                    </div>
                    <div class="radiogroup-item-wrapper">
                        <RadioGroupItem class="radiogroup-item" value="good" id="rating-good">
                            <RadioGroupIndicator class="radiogroup-indicator" />
                        </RadioGroupItem>
                        <label for="rating-good">"Good"</label>
                    </div>
                    <div class="radiogroup-item-wrapper">
                        <RadioGroupItem class="radiogroup-item" value="excellent" id="rating-excellent">
                            <RadioGroupIndicator class="radiogroup-indicator" />
                        </RadioGroupItem>
                        <label for="rating-excellent">"Excellent"</label>
                    </div>
                </RadioGroupRoot>
                <button type="submit" class="trigger-button" style="margin-top: 1rem">
                    "Submit Rating"
                </button>
            </form>
        </div>
    }
}
