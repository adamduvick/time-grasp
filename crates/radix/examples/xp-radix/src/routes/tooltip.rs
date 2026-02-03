use leptos::prelude::*;
use radix::{
    TooltipArrow, TooltipContent, TooltipPortal, TooltipProvider, TooltipRoot, TooltipSide,
    TooltipTrigger,
};

/// Tooltip Primitive
///
/// RADIX PROVIDES:
/// - Show/hide on hover and focus
/// - Configurable delay (delayDuration)
/// - Skip delay when moving between tooltips (skipDelayDuration)
/// - Positioning with side and align options
/// - Arrow component
/// - Portal rendering
/// - data-state="delayed-open" | "instant-open" | "closed"
/// - data-side and data-align attributes
///
/// USER MUST IMPLEMENT:
/// - Tooltip appearance (background, text, padding)
/// - Arrow styling
/// - Animations (optional, can use data-state)
///
/// REQUIRES: TooltipProvider wrapper for delay sharing

#[component]
pub fn TooltipExample() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=400>
            <h1>"Tooltip"</h1>
            <p>
                "Hover/focus hints. Radix handles show/hide timing, positioning, and "
                "accessibility. User styles the appearance. Note: Provider is required."
            </p>

            <div class="example-section">
                <h2>"Basic Tooltip"</h2>
                <TooltipRoot>
                    <TooltipTrigger>
                        <button class="trigger-button">"Hover me"</button>
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent class="tooltip-content" side_offset=5>
                            "This is a tooltip"
                            <TooltipArrow class="tooltip-arrow" />
                        </TooltipContent>
                    </TooltipPortal>
                </TooltipRoot>
                <p style="margin-top: 1rem; font-size: 0.875rem">
                    "Also try focusing with Tab - tooltips work on focus too"
                </p>
            </div>

            <div class="example-section">
                <h2>"Different Positions"</h2>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap">
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"top"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side=TooltipSide::Top side_offset=5>
                                "Tooltip on top"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"right"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side=TooltipSide::Right side_offset=5>
                                "Tooltip on right"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"bottom"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side=TooltipSide::Bottom side_offset=5>
                                "Tooltip on bottom"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"left"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side=TooltipSide::Left side_offset=5>
                                "Tooltip on left"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                </div>
            </div>

            <div class="example-section">
                <h2>"Skip Delay When Moving Between"</h2>
                <p style="font-size: 0.875rem; margin-bottom: 1rem">
                    "Hover over one button, wait for tooltip, then quickly move to another. "
                    "The second tooltip appears instantly (skipDelayDuration behavior)."
                </p>
                <div style="display: flex; gap: 1rem">
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"Button A"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side_offset=5>
                                "Tooltip for A"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"Button B"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side_offset=5>
                                "Tooltip for B"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                    <TooltipRoot>
                        <TooltipTrigger>
                            <button class="trigger-button">"Button C"</button>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side_offset=5>
                                "Tooltip for C"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                </div>
            </div>

            <div class="example-section">
                <h2>"On Non-Button Elements"</h2>
                <div style="display: flex; gap: 1rem; align-items: center">
                    <TooltipRoot>
                        <TooltipTrigger>
                            <span
                                style="cursor: help; border-bottom: 1px dotted currentColor"
                                tabindex="0"
                            >
                                "What is this?"
                            </span>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side_offset=5>
                                "Tooltips can be attached to any element"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>

                    <TooltipRoot>
                        <TooltipTrigger>
                            <span
                                style="display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 24px; border-radius: 50%; background: var(--color-accent); color: white; cursor: pointer"
                                tabindex="0"
                            >
                                "?"
                            </span>
                        </TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent class="tooltip-content" side_offset=5>
                                "Help icon tooltip"
                                <TooltipArrow class="tooltip-arrow" />
                            </TooltipContent>
                        </TooltipPortal>
                    </TooltipRoot>
                </div>
            </div>

            <div class="example-section">
                <h2>"Instant Open (No Delay)"</h2>
                <TooltipRoot delay_duration=0>
                    <TooltipTrigger>
                        <button class="trigger-button">"Instant tooltip"</button>
                    </TooltipTrigger>
                    <TooltipPortal>
                        <TooltipContent class="tooltip-content" side_offset=5>
                            "This appears immediately (delay_duration=0)"
                            <TooltipArrow class="tooltip-arrow" />
                        </TooltipContent>
                    </TooltipPortal>
                </TooltipRoot>
            </div>

            <div class="example-section">
                <h2>"Keyboard Navigation"</h2>
                <ul style="font-size: 0.875rem; line-height: 1.8">
                    <li><code>"Tab"</code>" - Focus trigger, shows tooltip instantly"</li>
                    <li><code>"Escape"</code>" - Close tooltip while focused"</li>
                    <li>"Mouse hover - Shows after delay (unless skip delay active)"</li>
                </ul>
            </div>
        </TooltipProvider>
    }
}
