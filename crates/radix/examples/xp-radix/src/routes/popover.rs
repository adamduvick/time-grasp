use leptos::prelude::*;
use radix::{
    PopoverAlign, PopoverArrow, PopoverClose, PopoverContent, PopoverPortal, PopoverRoot,
    PopoverSide, PopoverTrigger,
};

/// Popover Primitive
///
/// RADIX PROVIDES:
/// - Positioning relative to trigger (side, align options)
/// - Collision detection (flips to avoid viewport edges)
/// - Arrow component that points to trigger
/// - Portal rendering
/// - Focus management (optional)
/// - data-state="open" | "closed"
/// - data-side="top" | "right" | "bottom" | "left"
/// - data-align="start" | "center" | "end"
///
/// USER MUST IMPLEMENT:
/// - Content styling (background, border, padding)
/// - Arrow styling (fill color to match content)
/// - Any animations (can use data-state)

#[component]
pub fn PopoverExample() -> impl IntoView {
    view! {
        <h1>"Popover"</h1>
        <p>
            "Floating content anchored to a trigger. Radix handles positioning, "
            "collision detection, and optional focus management. User styles the "
            "appearance."
        </p>

        <div class="example-section">
            <h2>"Basic Popover"</h2>
            <PopoverRoot>
                <PopoverTrigger class="trigger-button">
                    "Show Info"
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent class="popover-content" side_offset=5>
                        <h3 style="margin-bottom: 0.5rem">"Popover Content"</h3>
                        <p style="margin: 0">
                            "This content is positioned relative to the trigger with "
                            "collision detection."
                        </p>
                        <PopoverArrow class="popover-arrow" />
                    </PopoverContent>
                </PopoverPortal>
            </PopoverRoot>
        </div>

        <div class="example-section">
            <h2>"Different Positions"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap">
                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"top"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent class="popover-content" side=PopoverSide::Top side_offset=5>
                            <p style="margin: 0">
                                "Positioned on "<strong>"top"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>

                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"right"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent class="popover-content" side=PopoverSide::Right side_offset=5>
                            <p style="margin: 0">
                                "Positioned on "<strong>"right"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>

                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"bottom"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent class="popover-content" side=PopoverSide::Bottom side_offset=5>
                            <p style="margin: 0">
                                "Positioned on "<strong>"bottom"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>

                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"left"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent class="popover-content" side=PopoverSide::Left side_offset=5>
                            <p style="margin: 0">
                                "Positioned on "<strong>"left"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>
            </div>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Note: Radix may flip position if there's not enough space"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Alignment"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap">
                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"align=start"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent
                            class="popover-content"
                            side=PopoverSide::Bottom
                            align=PopoverAlign::Start
                            side_offset=5
                        >
                            <p style="margin: 0">
                                "Aligned to "<strong>"start"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>

                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"align=center"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent
                            class="popover-content"
                            side=PopoverSide::Bottom
                            align=PopoverAlign::Center
                            side_offset=5
                        >
                            <p style="margin: 0">
                                "Aligned to "<strong>"center"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>

                <PopoverRoot>
                    <PopoverTrigger class="trigger-button">"align=end"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent
                            class="popover-content"
                            side=PopoverSide::Bottom
                            align=PopoverAlign::End
                            side_offset=5
                        >
                            <p style="margin: 0">
                                "Aligned to "<strong>"end"</strong>
                            </p>
                            <PopoverArrow class="popover-arrow" />
                        </PopoverContent>
                    </PopoverPortal>
                </PopoverRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"With Form Content"</h2>
            <PopoverRoot>
                <PopoverTrigger class="trigger-button">
                    "Update dimensions"
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent class="popover-content" side_offset=5>
                        <h3 style="margin-bottom: 0.5rem">"Dimensions"</h3>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.5rem">
                            <div>
                                <label for="width" style="display: block; font-size: 0.875rem">
                                    "Width"
                                </label>
                                <input
                                    id="width"
                                    type="number"
                                    value="100"
                                    style="width: 100%; padding: 0.25rem"
                                />
                            </div>
                            <div>
                                <label for="height" style="display: block; font-size: 0.875rem">
                                    "Height"
                                </label>
                                <input
                                    id="height"
                                    type="number"
                                    value="100"
                                    style="width: 100%; padding: 0.25rem"
                                />
                            </div>
                        </div>
                        <PopoverClose class="popover-close">"\u{00D7}"</PopoverClose>
                        <PopoverArrow class="popover-arrow" />
                    </PopoverContent>
                </PopoverPortal>
            </PopoverRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled State"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Popover can be controlled via open/onOpenChange props for programmatic "
                "control."
            </p>
            <PopoverRoot>
                <PopoverTrigger class="trigger-button">
                    "Click me"
                </PopoverTrigger>
                <PopoverPortal>
                    <PopoverContent class="popover-content" side_offset=5>
                        <p style="margin: 0">
                            "This popover can be controlled externally via open/onOpenChange "
                            "props."
                        </p>
                        <PopoverArrow class="popover-arrow" />
                    </PopoverContent>
                </PopoverPortal>
            </PopoverRoot>
        </div>
    }
}
