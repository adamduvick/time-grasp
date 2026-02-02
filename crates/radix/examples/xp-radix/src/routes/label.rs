use leptos::prelude::*;
use radix::Label;

#[component]
pub fn LabelExample() -> impl IntoView {
    view! {
        <h1>"Label"</h1>
        <p>
            "Accessible label with click-to-focus behavior. Ensures clicking the "
            "label focuses the associated control. Simple but ensures consistency."
        </p>

        <div class="example-section">
            <h2>"Basic Label with Input"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem">
                <Label class="label-root" html_for="firstName">
                    "First name"
                </Label>
                <input
                    class="form-input"
                    type="text"
                    id="firstName"
                    placeholder="Enter your first name"
                />
            </div>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Click the label text to focus the input"
            </p>
        </div>

        <div class="example-section">
            <h2>"Label with Different Controls"</h2>
            <div style="display: flex; flex-direction: column; gap: 1.5rem">
                <div style="display: flex; flex-direction: column; gap: 0.5rem">
                    <Label class="label-root" html_for="email">
                        "Email address"
                    </Label>
                    <input
                        class="form-input"
                        type="email"
                        id="email"
                        placeholder="you@example.com"
                    />
                </div>

                <div style="display: flex; flex-direction: column; gap: 0.5rem">
                    <Label class="label-root" html_for="country">
                        "Country"
                    </Label>
                    <select class="form-input" id="country">
                        <option value="">"Select a country"</option>
                        <option value="us">"United States"</option>
                        <option value="uk">"United Kingdom"</option>
                        <option value="ca">"Canada"</option>
                    </select>
                </div>

                <div style="display: flex; flex-direction: column; gap: 0.5rem">
                    <Label class="label-root" html_for="message">
                        "Message"
                    </Label>
                    <textarea
                        class="form-input"
                        id="message"
                        rows="3"
                        placeholder="Write your message..."
                    />
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Inline Label (Checkbox Style)"</h2>
            <div style="display: flex; align-items: center; gap: 0.5rem">
                <input type="checkbox" id="terms" />
                <Label class="label-root" html_for="terms">
                    "I agree to the terms and conditions"
                </Label>
            </div>
        </div>

        <div class="example-section">
            <h2>"Wrapped Control (No htmlFor needed)"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "When the control is a child of the label, no htmlFor/id is needed."
            </p>
            <Label class="label-root" style="display: flex; flex-direction: column; gap: 0.5rem">
                "Username"
                <input class="form-input" type="text" placeholder="Enter username" />
            </Label>
        </div>

        <div class="example-section">
            <h2>"Required Field Indicator"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem">
                <Label class="label-root" html_for="required-field">
                    "Password "
                    <span style="color: #ef4444">"*"</span>
                </Label>
                <input
                    class="form-input"
                    type="password"
                    id="required-field"
                    required=true
                />
            </div>
        </div>

        <div class="example-section">
            <h2>"With Helper Text"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.25rem">
                <Label class="label-root" html_for="phone">
                    "Phone number"
                </Label>
                <input
                    class="form-input"
                    type="tel"
                    id="phone"
                    placeholder="+1 (555) 000-0000"
                    aria-describedby="phone-hint"
                />
                <span
                    id="phone-hint"
                    style="font-size: 0.75rem; color: var(--color-text-muted)"
                >
                    "Include country code for international numbers"
                </span>
            </div>
        </div>

        <div class="example-section">
            <h2>"Disabled State"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem">
                <Label
                    class="label-root"
                    html_for="disabled-input"
                    style="opacity: 0.5"
                >
                    "Disabled field"
                </Label>
                <input
                    class="form-input"
                    type="text"
                    id="disabled-input"
                    disabled=true
                    value="Cannot edit this"
                />
            </div>
        </div>

        <div class="example-section">
            <h2>"Why Use Radix Label?"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <strong>"Consistent behavior:"</strong>
                    " Click-to-focus works the same across all browsers"
                </li>
                <li>
                    <strong>"Proper association:"</strong>
                    " Ensures accessibility with htmlFor/id linking"
                </li>
                <li>
                    <strong>"Flexible:"</strong>
                    " Works with native controls and Radix primitives"
                </li>
                <li>
                    <strong>"Simple:"</strong>
                    " Just renders a styled label element with correct semantics"
                </li>
            </ul>
        </div>
    }
}
