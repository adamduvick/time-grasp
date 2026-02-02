use leptos::prelude::*;
use radix::{
    AccordionContent, AccordionHeader, AccordionItem, AccordionRoot, AccordionTrigger,
    AccordionType,
};

/// Accordion Primitive
///
/// RADIX PROVIDES:
/// - Expand/collapse state management
/// - Single or multiple open items (type="single" | "multiple")
/// - Keyboard navigation (arrow keys between headers)
/// - data-state="open" | "closed" on items and content
/// - data-disabled on disabled items
/// - Proper ARIA: role, aria-expanded, aria-controls
/// - Optional collapsible (allow all closed in single mode)
///
/// USER MUST IMPLEMENT:
/// - Header/trigger styling
/// - Content container styling
/// - Open/close indicators (chevrons, +/- icons)
/// - Animations (can use data-state, or CSS transitions)

#[component]
pub fn AccordionExample() -> impl IntoView {
    view! {
        <h1>"Accordion"</h1>
        <p>
            "Collapsible sections. Radix handles expand/collapse logic, keyboard nav, "
            "and ARIA. User styles headers, content, and indicators."
        </p>

        <div class="example-section">
            <h2>"Single Mode (only one open at a time)"</h2>
            <AccordionRoot
                accordion_type=AccordionType::Single
                value=RwSignal::new(vec!["item-1".to_string()])
                collapsible=true
            >
                <AccordionItem class="accordion-item" value="item-1">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Is it accessible?"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Yes. It adheres to the WAI-ARIA design pattern."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="item-2">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Is it unstyled?"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Yes. It's unstyled by default, giving you freedom over the look "
                        "and feel."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="item-3">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Can it be animated?"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Yes! You can animate the Accordion with CSS or JavaScript. This "
                        "example uses CSS data-state selectors."
                    </AccordionContent>
                </AccordionItem>
            </AccordionRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys to navigate between headers, Enter/Space to toggle"
            </p>
        </div>

        <div class="example-section">
            <h2>"Multiple Mode (multiple can be open)"</h2>
            <AccordionRoot
                accordion_type=AccordionType::Multiple
                value=RwSignal::new(vec!["item-a".to_string(), "item-b".to_string()])
                class="accordion-root"
            >
                <AccordionItem class="accordion-item" value="item-a">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Section A (open by default)"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Content for section A. In multiple mode, opening this doesn't "
                        "close others."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="item-b">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Section B (also open by default)"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Content for section B. Multiple sections can be expanded "
                        "simultaneously."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="item-c">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Section C (closed by default)"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Content for section C."
                    </AccordionContent>
                </AccordionItem>
            </AccordionRoot>
        </div>

        <div class="example-section">
            <h2>"With Disabled Items"</h2>
            <AccordionRoot
                accordion_type=AccordionType::Single
                value=RwSignal::new(vec!["enabled-1".to_string()])
                class="accordion-root"
            >
                <AccordionItem class="accordion-item" value="enabled-1">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Enabled Item"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "This item can be opened and closed."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="disabled-1" disabled=true>
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Disabled Item (can't be toggled)"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "You shouldn't see this."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="enabled-2">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Another Enabled Item"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "This item also works normally."
                    </AccordionContent>
                </AccordionItem>
            </AccordionRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Note: Keyboard navigation skips the disabled item"
            </p>
        </div>

        <div class="example-section">
            <h2>"Non-Collapsible (always one open)"</h2>
            <AccordionRoot
                accordion_type=AccordionType::Single
                value=RwSignal::new(vec!["required-1".to_string()])
                class="accordion-root"
            >
                <AccordionItem class="accordion-item" value="required-1">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "First Item"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "Try clicking this header again - it won't close because "
                        "collapsible is not set."
                    </AccordionContent>
                </AccordionItem>

                <AccordionItem class="accordion-item" value="required-2">
                    <AccordionHeader>
                        <AccordionTrigger class="accordion-trigger">
                            "Second Item"
                            <span class="accordion-chevron">"\u{25BC}"</span>
                        </AccordionTrigger>
                    </AccordionHeader>
                    <AccordionContent class="accordion-content">
                        "At least one item must always be open."
                    </AccordionContent>
                </AccordionItem>
            </AccordionRoot>
        </div>
    }
}
