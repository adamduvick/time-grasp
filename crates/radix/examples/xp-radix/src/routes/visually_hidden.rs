use leptos::prelude::*;
use radix::VisuallyHidden;

/// VisuallyHidden Primitive (Utility)
///
/// RADIX PROVIDES:
/// - Hides content visually while keeping it accessible to screen readers
/// - Uses CSS technique that maintains accessibility
/// - Content is still focusable if interactive
/// - Useful for: screen reader only text, skip links, icon button labels
///
/// USER MUST IMPLEMENT:
/// - Decide what content needs to be visually hidden
/// - Context for when to use vs aria-label
///
/// CSS TECHNIQUE:
/// Position absolute, 1x1px, overflow hidden, clip - not display:none

#[component]
pub fn VisuallyHiddenExample() -> impl IntoView {
    view! {
        <h1>"VisuallyHidden"</h1>
        <p>
            "Utility to hide content visually while keeping it accessible to screen "
            "readers. Content is hidden from sighted users but announced by assistive "
            "technology."
        </p>

        <div class="example-section">
            <h2>"Icon Button with Hidden Label"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Screen readers will announce \"Delete item\" even though only the icon "
                "is visible."
            </p>
            <button class="trigger-button" style="padding: 0.5rem">
                "\u{1F5D1}\u{FE0F}"
                <VisuallyHidden>"Delete item"</VisuallyHidden>
            </button>
            <button class="trigger-button" style="padding: 0.5rem; margin-left: 0.5rem">
                "\u{270F}\u{FE0F}"
                <VisuallyHidden>"Edit item"</VisuallyHidden>
            </button>
            <button class="trigger-button" style="padding: 0.5rem; margin-left: 0.5rem">
                "\u{2B50}"
                <VisuallyHidden>"Add to favorites"</VisuallyHidden>
            </button>
        </div>

        <div class="example-section">
            <h2>"Additional Context for Screen Readers"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Sighted users see \"Read more\", screen readers hear \"Read more about "
                "web accessibility best practices\"."
            </p>
            <a href="#" on:click=|e| e.prevent_default() class="trigger-button">
                "Read more"
                <VisuallyHidden>" about web accessibility best practices"</VisuallyHidden>
            </a>
        </div>

        <div class="example-section">
            <h2>"Skip Link"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Skip links are often visually hidden until focused. This example shows "
                "the concept (note: this one is always visible for demo purposes)."
            </p>
            <a href="#main-content" on:click=|e| e.prevent_default() class="trigger-button">
                "Skip to main content"
            </a>
            <p style="font-size: 0.75rem; color: var(--color-text-muted); margin-top: 0.5rem">
                "In real usage, this would be visually hidden until focused."
            </p>
        </div>

        <div class="example-section">
            <h2>"Form Field with Hidden Instructions"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem">
                <label for="card" class="label-root">
                    "Card Number"
                </label>
                <input
                    class="form-input"
                    type="text"
                    id="card"
                    placeholder="1234 5678 9012 3456"
                    aria-describedby="card-hint"
                />
                <VisuallyHidden>
                    <span id="card-hint">
                        "Enter your 16-digit card number without spaces or dashes"
                    </span>
                </VisuallyHidden>
            </div>
            <p style="font-size: 0.75rem; color: var(--color-text-muted); margin-top: 0.5rem">
                "Screen readers will announce the hidden hint when the input is focused."
            </p>
        </div>

        <div class="example-section">
            <h2>"Table with Hidden Headers"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Sometimes visual design omits table headers, but screen readers need them."
            </p>
            <table style="width: 100%; border-collapse: collapse">
                <thead>
                    <tr>
                        <th scope="col">
                            <VisuallyHidden>"Product Name"</VisuallyHidden>
                        </th>
                        <th scope="col">
                            <VisuallyHidden>"Price"</VisuallyHidden>
                        </th>
                        <th scope="col">
                            <VisuallyHidden>"Actions"</VisuallyHidden>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem; border-bottom: 1px solid var(--color-border)">
                            "Widget Pro"
                        </td>
                        <td style="padding: 0.5rem; border-bottom: 1px solid var(--color-border)">
                            "$49.99"
                        </td>
                        <td style="padding: 0.5rem; border-bottom: 1px solid var(--color-border)">
                            <button class="trigger-button" style="padding: 0.25rem 0.5rem">
                                "Buy"
                            </button>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Gadget Max"</td>
                        <td style="padding: 0.5rem">"$79.99"</td>
                        <td style="padding: 0.5rem">
                            <button class="trigger-button" style="padding: 0.25rem 0.5rem">
                                "Buy"
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="example-section">
            <h2>"Loading State Announcement"</h2>
            <button class="trigger-button">
                "Save"
                // In a real app, this would be conditionally rendered when loading:
                // <VisuallyHidden aria-live="polite">"Loading, please wait..."</VisuallyHidden>
            </button>
        </div>

        <div class="example-section">
            <h2>"How It Works (CSS)"</h2>
            <pre style="background: var(--color-bg); padding: 1rem; border-radius: var(--radius); overflow: auto; font-size: 0.875rem">
"/* VisuallyHidden uses this CSS technique */
position: absolute;
border: 0;
width: 1px;
height: 1px;
padding: 0;
margin: -1px;
overflow: hidden;
clip: rect(0, 0, 0, 0);
white-space: nowrap;
word-wrap: normal;"
            </pre>
            <p style="font-size: 0.875rem; margin-top: 1rem">
                "Unlike "<code>"display: none"</code>" or "<code>"visibility: hidden"</code>", "
                "this technique keeps content accessible to screen readers."
            </p>
        </div>

        <div class="example-section">
            <h2>"When to Use"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>"Icon-only buttons that need accessible names"</li>
                <li>"Additional context for screen reader users"</li>
                <li>"Skip navigation links"</li>
                <li>"Visually redundant table headers"</li>
                <li>"Live region announcements"</li>
                <li>"Form instructions that clutter visual design"</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"VisuallyHidden vs aria-label"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Use"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"VisuallyHidden"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"aria-label"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Complex content (HTML)"</td>
                        <td style="padding: 0.5rem">"\u{2713} Supports markup"</td>
                        <td style="padding: 0.5rem">"\u{2717} String only"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Simple button label"</td>
                        <td style="padding: 0.5rem">"Works"</td>
                        <td style="padding: 0.5rem">"\u{2713} Simpler"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Focusable content"</td>
                        <td style="padding: 0.5rem">"\u{2713} Content still focusable"</td>
                        <td style="padding: 0.5rem">"\u{2717} No focusable content"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
