use leptos::prelude::*;
use radix::{Separator, SeparatorOrientation};

#[component]
pub fn SeparatorExample() -> impl IntoView {
    view! {
        <h1>"Separator"</h1>
        <p>
            "Visual divider between content sections. Radix handles ARIA attributes. "
            "User styles the visual appearance."
        </p>

        // Horizontal Separator
        <div class="example-section">
            <h2>"Horizontal Separator"</h2>
            <div>
                <p>"Section one content"</p>
                <Separator class="separator-horizontal" />
                <p>"Section two content"</p>
                <Separator class="separator-horizontal" />
                <p>"Section three content"</p>
            </div>
        </div>

        // Vertical Separator
        <div class="example-section">
            <h2>"Vertical Separator"</h2>
            <div style="display: flex; align-items: center; height: 40px">
                <span>"Home"</span>
                <Separator
                    class="separator-vertical"
                    orientation=SeparatorOrientation::Vertical
                />
                <span>"About"</span>
                <Separator
                    class="separator-vertical"
                    orientation=SeparatorOrientation::Vertical
                />
                <span>"Contact"</span>
            </div>
        </div>

        // In a Card
        <div class="example-section">
            <h2>"In a Card"</h2>
            <div style="background: var(--color-bg); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 1rem; max-width: 300px">
                <h3 style="margin: 0 0 0.5rem">"Radix Primitives"</h3>
                <p style="margin: 0; font-size: 0.875rem">
                    "An open-source UI component library."
                </p>
                <Separator class="separator-horizontal" />
                <div style="display: flex; gap: 1rem; font-size: 0.875rem">
                    <span>"GitHub"</span>
                    <Separator
                        class="separator-vertical"
                        orientation=SeparatorOrientation::Vertical
                        style="height: auto; align-self: stretch"
                    />
                    <span>"Discord"</span>
                    <Separator
                        class="separator-vertical"
                        orientation=SeparatorOrientation::Vertical
                        style="height: auto; align-self: stretch"
                    />
                    <span>"Twitter"</span>
                </div>
            </div>
        </div>

        // Decorative vs Non-Decorative
        <div class="example-section">
            <h2>"Decorative vs Non-Decorative"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Set "<code>"decorative=true"</code>" when the separator is purely "
                "visual and doesn't represent a semantic break."
            </p>
            <div>
                <p>"Semantic separator (role=\"separator\"):"</p>
                <Separator class="separator-horizontal" />
                <p>"Decorative separator (role=\"none\"):"</p>
                <Separator class="separator-horizontal" decorative=true />
            </div>
        </div>

        // With Label
        <div class="example-section">
            <h2>"With Label"</h2>
            <div style="display: flex; align-items: center; gap: 1rem">
                <Separator
                    class="separator-horizontal"
                    decorative=true
                    style="flex: 1"
                />
                <span style="color: var(--color-text-muted); font-size: 0.875rem">
                    "OR"
                </span>
                <Separator
                    class="separator-horizontal"
                    decorative=true
                    style="flex: 1"
                />
            </div>
        </div>

        // Menu Example
        <div class="example-section">
            <h2>"Menu Example"</h2>
            <div style="background: var(--color-bg); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 0.25rem; width: 200px">
                <div style="padding: 0.5rem 0.75rem">"Cut"</div>
                <div style="padding: 0.5rem 0.75rem">"Copy"</div>
                <div style="padding: 0.5rem 0.75rem">"Paste"</div>
                <Separator class="separator-horizontal" style="margin: 0.25rem 0" />
                <div style="padding: 0.5rem 0.75rem">"Select All"</div>
            </div>
        </div>

        // Custom Styles
        <div class="example-section">
            <h2>"Custom Styles"</h2>
            <div style="display: flex; flex-direction: column; gap: 1rem">
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">"Dashed:"</p>
                    <Separator style="height: 1px; background: none; border-top: 1px dashed var(--color-border)" />
                </div>

                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">"Thick:"</p>
                    <Separator style="height: 4px; background: var(--color-accent); border-radius: 2px" />
                </div>

                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">"Gradient:"</p>
                    <Separator style="height: 2px; background: linear-gradient(90deg, transparent, var(--color-accent), transparent)" />
                </div>
            </div>
        </div>

        // Accessibility
        <div class="example-section">
            <h2>"Accessibility"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"role=\"separator\""</code>" - Indicates content break"
                </li>
                <li>
                    <code>"aria-orientation"</code>" - Horizontal or vertical"
                </li>
                <li>
                    <code>"decorative"</code>" - Sets role=\"none\" for purely visual separators"
                </li>
            </ul>
        </div>
    }
}
