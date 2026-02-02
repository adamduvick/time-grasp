use leptos::prelude::*;
use radix::{ToggleGroupItem, ToggleGroupOrientation, ToggleGroupRoot, ToggleGroupType};

/// ToggleGroup Primitive
///
/// RADIX PROVIDES:
/// - Single or multiple selection within group
/// - Keyboard navigation (arrow keys, tab)
/// - data-state="on" | "off" on items
/// - data-orientation="horizontal" | "vertical"
/// - Roving tabindex
/// - Controlled and uncontrolled modes
///
/// USER MUST IMPLEMENT:
/// - Group container styling
/// - Item styling for both states
/// - Focus ring styling
///
/// DIFFERENT FROM RADIOGROUP:
/// - ToggleGroup allows deselection in single mode
/// - Can have multiple selection mode
/// - Visual metaphor is buttons, not radio circles

#[component]
pub fn ToggleGroupExample() -> impl IntoView {
    let alignment = RwSignal::new(vec!["center".to_string()]);
    let formats = RwSignal::new(Vec::<String>::new());
    let controlled_alignment = RwSignal::new(vec!["center".to_string()]);
    let vertical_value = RwSignal::new(vec!["list".to_string()]);
    let disabled_value = RwSignal::new(vec!["weekly".to_string()]);
    let icon_value = RwSignal::new(vec!["play".to_string()]);

    view! {
        <h1>"ToggleGroup"</h1>
        <p>
            "Group of toggles with single or multiple selection. Like RadioGroup but "
            "allows deselection and multiple mode. Uses toggle button visual metaphor."
        </p>

        <div class="example-section">
            <h2>"Single Selection"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Single
                value=alignment
                aria_label="Text alignment"
            >
                <ToggleGroupItem class="togglegroup-item" value="left" aria_label="Left aligned">
                    "\u{2190}"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="center" aria_label="Center aligned">
                    "\u{2194}"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="right" aria_label="Right aligned">
                    "\u{2192}"
                </ToggleGroupItem>
            </ToggleGroupRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Click the same button again to deselect (unlike RadioGroup)"
            </p>
        </div>

        <div class="example-section">
            <h2>"Multiple Selection"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Multiple
                value=formats
                aria_label="Text formatting"
            >
                <ToggleGroupItem class="togglegroup-item" value="bold" aria_label="Bold">
                    <strong>"B"</strong>
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="italic" aria_label="Italic">
                    <em>"I"</em>
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="underline" aria_label="Underline">
                    <span style="text-decoration: underline">"U"</span>
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="strikethrough" aria_label="Strikethrough">
                    <span style="text-decoration: line-through">"S"</span>
                </ToggleGroupItem>
            </ToggleGroupRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "Selected: "
                {move || {
                    let f = formats.get();
                    if f.is_empty() { "(none)".to_string() } else { f.join(", ") }
                }}
            </p>
        </div>

        <div class="example-section">
            <h2>"Controlled Single Selection"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Single
                value=controlled_alignment
                aria_label="View mode"
            >
                <ToggleGroupItem class="togglegroup-item" value="left">
                    "Left"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="center">
                    "Center"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="right">
                    "Right"
                </ToggleGroupItem>
            </ToggleGroupRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "Value: "
                <strong>
                    {move || controlled_alignment.get().first().cloned().unwrap_or_default()}
                </strong>
            </p>
            <p style="font-size: 0.875rem">
                "This example allows deselection in single mode."
            </p>
        </div>

        <div class="example-section">
            <h2>"Vertical Orientation"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Single
                orientation=ToggleGroupOrientation::Vertical
                value=vertical_value
                aria_label="View type"
                style="flex-direction: column; width: fit-content"
            >
                <ToggleGroupItem
                    class="togglegroup-item"
                    value="grid"
                    style="width: 100%; justify-content: flex-start"
                >
                    "\u{25A6} Grid"
                </ToggleGroupItem>
                <ToggleGroupItem
                    class="togglegroup-item"
                    value="list"
                    style="width: 100%; justify-content: flex-start"
                >
                    "\u{2261} List"
                </ToggleGroupItem>
                <ToggleGroupItem
                    class="togglegroup-item"
                    value="columns"
                    style="width: 100%; justify-content: flex-start"
                >
                    "\u{2016} Columns"
                </ToggleGroupItem>
            </ToggleGroupRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Up/Down arrows navigate (not Left/Right)"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Disabled Items"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Single
                value=disabled_value
                aria_label="Report frequency"
            >
                <ToggleGroupItem class="togglegroup-item" value="daily">
                    "Daily"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="weekly">
                    "Weekly"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="monthly" disabled=true>
                    "Monthly"
                </ToggleGroupItem>
            </ToggleGroupRoot>
        </div>

        <div class="example-section">
            <h2>"Icon Buttons"</h2>
            <ToggleGroupRoot
                class="togglegroup-root"
                group_type=ToggleGroupType::Single
                value=icon_value
                aria_label="Playback controls"
            >
                <ToggleGroupItem class="togglegroup-item" value="prev" aria_label="Previous">
                    "\u{23EE}"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="play" aria_label="Play">
                    "\u{25B6}"
                </ToggleGroupItem>
                <ToggleGroupItem class="togglegroup-item" value="next" aria_label="Next">
                    "\u{23ED}"
                </ToggleGroupItem>
            </ToggleGroupRoot>
        </div>

        <div class="example-section">
            <h2>"ToggleGroup vs RadioGroup"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"ToggleGroup"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"RadioGroup"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Deselection"</td>
                        <td style="padding: 0.5rem">"Allowed (single mode)"</td>
                        <td style="padding: 0.5rem">"Not allowed"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Multiple selection"</td>
                        <td style="padding: 0.5rem">"Yes (type=\"multiple\")"</td>
                        <td style="padding: 0.5rem">"No"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Visual metaphor"</td>
                        <td style="padding: 0.5rem">"Toggle buttons"</td>
                        <td style="padding: 0.5rem">"Radio circles"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"ARIA"</td>
                        <td style="padding: 0.5rem">"aria-pressed"</td>
                        <td style="padding: 0.5rem">"role=\"radiogroup\""</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
