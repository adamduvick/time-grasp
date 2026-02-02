use leptos::prelude::*;
use radix::{CollapsibleContent, CollapsibleRoot, CollapsibleTrigger};

/// Collapsible Primitive
///
/// RADIX PROVIDES:
/// - Simple open/closed state management
/// - data-state="open" | "closed" on Root and Content
/// - data-disabled when disabled
/// - Controlled and uncontrolled modes
/// - aria-expanded on trigger
/// - aria-controls association
///
/// USER MUST IMPLEMENT:
/// - Trigger styling (often just a button)
/// - Content container styling
/// - Open/close indicator (chevron, +/-, etc.)
/// - Animations (CSS transitions on data-state)
///
/// DIFFERENCE FROM ACCORDION:
/// - Collapsible is for single items
/// - Accordion is for groups with single/multiple constraints

#[component]
pub fn CollapsibleExample() -> impl IntoView {
    let controlled_open = RwSignal::new(false);

    view! {
        <h1>"Collapsible"</h1>
        <p>
            "Simple expand/collapse container. Unlike Accordion, this is for single "
            "collapsible regions without group constraints."
        </p>

        <div class="example-section">
            <h2>"Basic Collapsible"</h2>
            <CollapsibleRoot class="collapsible-root" open=RwSignal::new(false)>
                <CollapsibleTrigger class="collapsible-trigger">
                    <span>"Toggle content"</span>
                    <span class="collapsible-chevron">"\u{25BC}"</span>
                </CollapsibleTrigger>
                <CollapsibleContent class="collapsible-content">
                    <p>
                        "This content can be expanded or collapsed. Radix handles the "
                        "state, ARIA attributes, and data-state for styling."
                    </p>
                </CollapsibleContent>
            </CollapsibleRoot>
        </div>

        <div class="example-section">
            <h2>"Default Open"</h2>
            <CollapsibleRoot class="collapsible-root" open=RwSignal::new(true)>
                <CollapsibleTrigger class="collapsible-trigger">
                    <span>"Repository files"</span>
                    <span class="collapsible-chevron">"\u{25BC}"</span>
                </CollapsibleTrigger>
                <CollapsibleContent class="collapsible-content">
                    <ul style="margin: 0; padding-left: 1.5rem">
                        <li>"README.md"</li>
                        <li>"package.json"</li>
                        <li>"tsconfig.json"</li>
                        <li>"src/"</li>
                    </ul>
                </CollapsibleContent>
            </CollapsibleRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled State"</h2>
            <div style="margin-bottom: 1rem">
                <button
                    class="trigger-button"
                    on:click=move |_| controlled_open.update(|o| *o = !*o)
                >
                    {move || if controlled_open.get() { "Close" } else { "Open" }}
                    " from outside"
                </button>
            </div>
            <CollapsibleRoot class="collapsible-root" open=controlled_open>
                <CollapsibleTrigger class="collapsible-trigger">
                    <span>"Controlled collapsible"</span>
                    <span class="collapsible-chevron">"\u{25BC}"</span>
                </CollapsibleTrigger>
                <CollapsibleContent class="collapsible-content">
                    <p>
                        "State: "
                        <strong>{move || if controlled_open.get() { "open" } else { "closed" }}</strong>
                    </p>
                    <p>"This collapsible is controlled via external state."</p>
                </CollapsibleContent>
            </CollapsibleRoot>
        </div>

        <div class="example-section">
            <h2>"Disabled"</h2>
            <CollapsibleRoot class="collapsible-root" open=RwSignal::new(false) disabled=true>
                <CollapsibleTrigger class="collapsible-trigger">
                    <span>"Can't toggle (disabled)"</span>
                    <span class="collapsible-chevron">"\u{25BC}"</span>
                </CollapsibleTrigger>
                <CollapsibleContent class="collapsible-content">
                    <p>"You shouldn't see this."</p>
                </CollapsibleContent>
            </CollapsibleRoot>
        </div>

        <div class="example-section">
            <h2>"Multiple Independent Collapsibles"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Unlike Accordion, multiple Collapsibles don't affect each other."
            </p>
            <div style="display: flex; flex-direction: column; gap: 0.5rem">
                <CollapsibleRoot class="collapsible-root" open=RwSignal::new(false)>
                    <CollapsibleTrigger class="collapsible-trigger">
                        <span>"Section A"</span>
                        <span class="collapsible-chevron">"\u{25BC}"</span>
                    </CollapsibleTrigger>
                    <CollapsibleContent class="collapsible-content">
                        "Content for Section A"
                    </CollapsibleContent>
                </CollapsibleRoot>

                <CollapsibleRoot class="collapsible-root" open=RwSignal::new(false)>
                    <CollapsibleTrigger class="collapsible-trigger">
                        <span>"Section B"</span>
                        <span class="collapsible-chevron">"\u{25BC}"</span>
                    </CollapsibleTrigger>
                    <CollapsibleContent class="collapsible-content">
                        "Content for Section B"
                    </CollapsibleContent>
                </CollapsibleRoot>

                <CollapsibleRoot class="collapsible-root" open=RwSignal::new(false)>
                    <CollapsibleTrigger class="collapsible-trigger">
                        <span>"Section C"</span>
                        <span class="collapsible-chevron">"\u{25BC}"</span>
                    </CollapsibleTrigger>
                    <CollapsibleContent class="collapsible-content">
                        "Content for Section C"
                    </CollapsibleContent>
                </CollapsibleRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"Collapsible vs Accordion"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Collapsible"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Accordion"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Items"</td>
                        <td style="padding: 0.5rem">"Single"</td>
                        <td style="padding: 0.5rem">"Multiple"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Group constraints"</td>
                        <td style="padding: 0.5rem">"None"</td>
                        <td style="padding: 0.5rem">"Single/Multiple mode"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Keyboard nav"</td>
                        <td style="padding: 0.5rem">"Just toggle"</td>
                        <td style="padding: 0.5rem">"Arrow keys between items"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
