use leptos::prelude::*;
use radix::{
    SelectContent, SelectGroup, SelectIcon, SelectItem, SelectItemIndicator, SelectItemText,
    SelectLabel, SelectPortal, SelectRoot, SelectScrollDownButton, SelectScrollUpButton,
    SelectSeparator, SelectTrigger, SelectValue, SelectViewport,
};

/// Select Primitive
///
/// RADIX PROVIDES:
/// - Custom dropdown select (not native <select>)
/// - Typeahead (type to jump to matching option)
/// - Keyboard navigation (arrow keys, home/end, enter)
/// - Groups with labels
/// - Scroll buttons for long lists
/// - data-state="open" | "closed" on trigger/content
/// - data-highlighted on focused item
/// - data-disabled on disabled items
/// - Proper ARIA: listbox pattern
/// - Value/Viewport/Content separation for positioning
///
/// USER MUST IMPLEMENT:
/// - Trigger button styling
/// - Dropdown content styling
/// - Item styling (including highlighted state)
/// - Scroll button styling
/// - Icons (chevrons, checkmarks)

#[component]
pub fn SelectExample() -> impl IntoView {
    // State for controlled example
    let fruit = RwSignal::new(String::new());

    view! {
        <h1>"Select"</h1>
        <p>
            "Custom dropdown select with typeahead and keyboard navigation. Radix "
            "handles positioning, selection, and ARIA. User styles all parts."
        </p>

        <div class="example-section">
            <h2>"Basic Select"</h2>
            <SelectRoot>
                <SelectTrigger class="select-trigger">
                    <SelectValue placeholder="Select a fruit..." />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class="select-content">
                        <SelectViewport class="select-viewport">
                            <SelectItem class="select-item" value="apple">
                                <SelectItemText>"Apple"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="banana">
                                <SelectItemText>"Banana"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="cherry">
                                <SelectItemText>"Cherry"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="grape">
                                <SelectItemText>"Grape"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </SelectRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Type to search (e.g., type \"b\" for Banana)"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Groups"</h2>
            <SelectRoot>
                <SelectTrigger class="select-trigger">
                    <SelectValue placeholder="Select a food..." />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class="select-content">
                        <SelectViewport class="select-viewport">
                            <SelectGroup>
                                <SelectLabel class="select-label">"Fruits"</SelectLabel>
                                <SelectItem class="select-item" value="apple2">
                                    <SelectItemText>"Apple"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem class="select-item" value="banana2">
                                    <SelectItemText>"Banana"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem class="select-item" value="orange">
                                    <SelectItemText>"Orange"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                            </SelectGroup>
                            <SelectSeparator class="select-separator" />
                            <SelectGroup>
                                <SelectLabel class="select-label">"Vegetables"</SelectLabel>
                                <SelectItem class="select-item" value="carrot">
                                    <SelectItemText>"Carrot"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem class="select-item" value="potato">
                                    <SelectItemText>"Potato"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                                <SelectItem class="select-item" value="broccoli">
                                    <SelectItemText>"Broccoli"</SelectItemText>
                                    <SelectItemIndicator class="select-item-indicator">
                                        "✓"
                                    </SelectItemIndicator>
                                </SelectItem>
                            </SelectGroup>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </SelectRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled Value"</h2>
            <SelectRoot
                value=fruit
                on_value_change=Callback::new(move |v| fruit.set(v))
            >
                <SelectTrigger class="select-trigger">
                    <SelectValue placeholder="Choose..." />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class="select-content">
                        <SelectViewport class="select-viewport">
                            <SelectItem class="select-item" value="strawberry">
                                <SelectItemText>"Strawberry"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="blueberry">
                                <SelectItemText>"Blueberry"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="raspberry">
                                <SelectItemText>"Raspberry"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </SelectRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "Selected: "
                <strong>
                    {move || {
                        let v = fruit.get();
                        if v.is_empty() { "(none)".to_string() } else { v }
                    }}
                </strong>
            </p>
            <button
                class="trigger-button"
                style="margin-top: 0.5rem"
                on:click=move |_| fruit.set(String::new())
            >
                "Clear"
            </button>
        </div>

        <div class="example-section">
            <h2>"With Disabled Items"</h2>
            <SelectRoot default_value="available">
                <SelectTrigger class="select-trigger">
                    <SelectValue />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class="select-content">
                        <SelectViewport class="select-viewport">
                            <SelectItem class="select-item" value="available">
                                <SelectItemText>"Available"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="out-of-stock" disabled=Signal::derive(|| true)>
                                <SelectItemText>"Out of Stock"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="preorder">
                                <SelectItemText>"Pre-order"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">
                                    "✓"
                                </SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                    </SelectContent>
                </SelectPortal>
            </SelectRoot>
        </div>

        <div class="example-section">
            <h2>"Long List with Scroll"</h2>
            <SelectRoot>
                <SelectTrigger class="select-trigger">
                    <SelectValue placeholder="Select a country..." />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
                <SelectPortal>
                    <SelectContent class="select-content">
                        <SelectScrollUpButton class="select-scroll-button">
                            "▲"
                        </SelectScrollUpButton>
                        <SelectViewport class="select-viewport select-viewport-scroll">
                            <SelectItem class="select-item" value="argentina">
                                <SelectItemText>"Argentina"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="australia">
                                <SelectItemText>"Australia"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="brazil">
                                <SelectItemText>"Brazil"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="canada">
                                <SelectItemText>"Canada"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="china">
                                <SelectItemText>"China"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="france">
                                <SelectItemText>"France"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="germany">
                                <SelectItemText>"Germany"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="india">
                                <SelectItemText>"India"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="italy">
                                <SelectItemText>"Italy"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="japan">
                                <SelectItemText>"Japan"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="mexico">
                                <SelectItemText>"Mexico"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="netherlands">
                                <SelectItemText>"Netherlands"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="south-korea">
                                <SelectItemText>"South Korea"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="spain">
                                <SelectItemText>"Spain"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="united-kingdom">
                                <SelectItemText>"United Kingdom"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem class="select-item" value="united-states">
                                <SelectItemText>"United States"</SelectItemText>
                                <SelectItemIndicator class="select-item-indicator">"✓"</SelectItemIndicator>
                            </SelectItem>
                        </SelectViewport>
                        <SelectScrollDownButton class="select-scroll-button">
                            "▼"
                        </SelectScrollDownButton>
                    </SelectContent>
                </SelectPortal>
            </SelectRoot>
        </div>

        <div class="example-section">
            <h2>"Disabled Select"</h2>
            <SelectRoot disabled=Signal::derive(|| true)>
                <SelectTrigger class="select-trigger">
                    <SelectValue placeholder="Disabled..." />
                    <SelectIcon class="select-icon">"▼"</SelectIcon>
                </SelectTrigger>
            </SelectRoot>
        </div>
    }
}
