use leptos::prelude::*;
use radix::{PopperAlign, PopperAnchor, PopperArrow, PopperContent, PopperRoot, PopperSide};

/// Popper Primitive
///
/// A low-level positioning primitive that handles floating UI placement.
/// This is NOT an accessibility primitive - it's the positioning engine
/// used internally by Tooltip, Popover, DropdownMenu, Select, etc.
///
/// RADIX PROVIDES:
/// - Automatic positioning relative to an anchor element
/// - Side and alignment options (top/right/bottom/left + start/center/end)
/// - Collision detection and avoidance
/// - Arrow positioning with automatic rotation
/// - CSS custom properties for available space
///
/// KEY INSIGHT - Arrow Height in Offset:
/// The sideOffset is automatically increased by the arrow's height!
/// offset = sideOffset + arrowHeight
/// This ensures the arrow tip touches where you expect.

#[component]
pub fn PopperExample() -> impl IntoView {
    let (side, set_side) = signal(PopperSide::Top);
    let (align, set_align) = signal(PopperAlign::Center);
    let (side_offset, set_side_offset) = signal(5i32);
    let (align_offset, set_align_offset) = signal(0i32);
    let (avoid_collisions, set_avoid_collisions) = signal(true);

    view! {
        <h1>"Popper"</h1>
        <p>
            "Low-level positioning primitive built on floating UI concepts. Handles anchor-relative "
            "positioning, collision detection, and arrow placement. Used internally by "
            "Tooltip, Popover, DropdownMenu, etc."
        </p>

        <div class="example-section">
            <h2>"Basic Positioning"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "The Popper positions content relative to an anchor. Content is always visible "
                "here - in real usage you'd control visibility separately."
            </p>
            <div style="display: flex; justify-content: center; padding: 4rem 2rem; background: var(--color-bg); border-radius: var(--radius)">
                <PopperRoot>
                    <PopperAnchor>
                        <button class="trigger-button">"Anchor Element"</button>
                    </PopperAnchor>
                    <PopperContent side=PopperSide::Top side_offset=5 class="popper-content">
                        "Popper content"
                        <PopperArrow class="popper-arrow" />
                    </PopperContent>
                </PopperRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"Interactive Positioning Controls"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Experiment with different positioning options."
            </p>

            <div style="display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 1rem">
                <label style="display: flex; align-items: center; gap: 0.5rem">
                    "Side:"
                    <select
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_side.set(match value.as_str() {
                                "top" => PopperSide::Top,
                                "right" => PopperSide::Right,
                                "bottom" => PopperSide::Bottom,
                                "left" => PopperSide::Left,
                                _ => PopperSide::Top,
                            });
                        }
                        style="padding: 0.25rem"
                    >
                        <option value="top" selected=move || side.get() == PopperSide::Top>"top"</option>
                        <option value="right" selected=move || side.get() == PopperSide::Right>"right"</option>
                        <option value="bottom" selected=move || side.get() == PopperSide::Bottom>"bottom"</option>
                        <option value="left" selected=move || side.get() == PopperSide::Left>"left"</option>
                    </select>
                </label>

                <label style="display: flex; align-items: center; gap: 0.5rem">
                    "Align:"
                    <select
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            set_align.set(match value.as_str() {
                                "start" => PopperAlign::Start,
                                "center" => PopperAlign::Center,
                                "end" => PopperAlign::End,
                                _ => PopperAlign::Center,
                            });
                        }
                        style="padding: 0.25rem"
                    >
                        <option value="start" selected=move || align.get() == PopperAlign::Start>"start"</option>
                        <option value="center" selected=move || align.get() == PopperAlign::Center>"center"</option>
                        <option value="end" selected=move || align.get() == PopperAlign::End>"end"</option>
                    </select>
                </label>

                <label style="display: flex; align-items: center; gap: 0.5rem">
                    "Side Offset:"
                    <input
                        type="number"
                        prop:value=move || side_offset.get()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse() {
                                set_side_offset.set(v);
                            }
                        }
                        style="width: 60px; padding: 0.25rem"
                    />
                </label>

                <label style="display: flex; align-items: center; gap: 0.5rem">
                    "Align Offset:"
                    <input
                        type="number"
                        prop:value=move || align_offset.get()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse() {
                                set_align_offset.set(v);
                            }
                        }
                        style="width: 60px; padding: 0.25rem"
                    />
                </label>

                <label style="display: flex; align-items: center; gap: 0.5rem">
                    <input
                        type="checkbox"
                        prop:checked=move || avoid_collisions.get()
                        on:change=move |ev| {
                            set_avoid_collisions.set(event_target_checked(&ev));
                        }
                    />
                    "Avoid Collisions"
                </label>
            </div>

            <div style="display: flex; justify-content: center; padding: 6rem 2rem; background: var(--color-bg); border-radius: var(--radius)">
                <PopperRoot>
                    <PopperAnchor>
                        <button class="trigger-button" style="min-width: 120px">
                            "Anchor"
                        </button>
                    </PopperAnchor>
                    <PopperContent
                        side=side
                        align=align
                        side_offset=side_offset
                        align_offset=align_offset
                        avoid_collisions=avoid_collisions
                        class="popper-content"
                    >
                        {move || format!("side: {}", side.get().as_str())}
                        <br />
                        {move || format!("align: {}", align.get().as_str())}
                        <PopperArrow class="popper-arrow" />
                    </PopperContent>
                </PopperRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"All Positions Grid"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "All 12 possible side + align combinations."
            </p>
            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 1rem">
                // Top
                <div>
                    <h3 style="font-size: 0.875rem; margin-bottom: 0.5rem">"top"</h3>
                    <PositionDemo side=PopperSide::Top align=PopperAlign::Start label="start" />
                    <PositionDemo side=PopperSide::Top align=PopperAlign::Center label="center" />
                    <PositionDemo side=PopperSide::Top align=PopperAlign::End label="end" />
                </div>
                // Right
                <div>
                    <h3 style="font-size: 0.875rem; margin-bottom: 0.5rem">"right"</h3>
                    <PositionDemo side=PopperSide::Right align=PopperAlign::Start label="start" />
                    <PositionDemo side=PopperSide::Right align=PopperAlign::Center label="center" />
                    <PositionDemo side=PopperSide::Right align=PopperAlign::End label="end" />
                </div>
                // Bottom
                <div>
                    <h3 style="font-size: 0.875rem; margin-bottom: 0.5rem">"bottom"</h3>
                    <PositionDemo side=PopperSide::Bottom align=PopperAlign::Start label="start" />
                    <PositionDemo side=PopperSide::Bottom align=PopperAlign::Center label="center" />
                    <PositionDemo side=PopperSide::Bottom align=PopperAlign::End label="end" />
                </div>
                // Left
                <div>
                    <h3 style="font-size: 0.875rem; margin-bottom: 0.5rem">"left"</h3>
                    <PositionDemo side=PopperSide::Left align=PopperAlign::Start label="start" />
                    <PositionDemo side=PopperSide::Left align=PopperAlign::Center label="center" />
                    <PositionDemo side=PopperSide::Left align=PopperAlign::End label="end" />
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Arrow Offset Calculation"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                <strong>"Key insight:"</strong>" The actual offset includes arrow height!"
                <br />
                <code>"totalOffset = sideOffset + arrowHeight"</code>
                <br />
                "With sideOffset=5 and default arrow height=5, total offset is 10px."
            </p>
            <div style="display: flex; gap: 3rem; justify-content: center; padding: 4rem 2rem; background: var(--color-bg); border-radius: var(--radius)">
                <div style="text-align: center">
                    <PopperRoot>
                        <PopperAnchor>
                            <button class="trigger-button">"With Arrow"</button>
                        </PopperAnchor>
                        <PopperContent side=PopperSide::Top side_offset=5 class="popper-content">
                            "sideOffset=5"
                            <br />
                            "(total: 10px)"
                            <PopperArrow class="popper-arrow" />
                        </PopperContent>
                    </PopperRoot>
                    <div style="font-size: 0.75rem; margin-top: 0.5rem; color: var(--color-text-muted)">
                        "offset + arrow = 10px"
                    </div>
                </div>

                <div style="text-align: center">
                    <PopperRoot>
                        <PopperAnchor>
                            <button class="trigger-button">"No Arrow"</button>
                        </PopperAnchor>
                        <PopperContent side=PopperSide::Top side_offset=5 class="popper-content">
                            "sideOffset=5"
                            <br />
                            "(total: 5px)"
                        </PopperContent>
                    </PopperRoot>
                    <div style="font-size: 0.75rem; margin-top: 0.5rem; color: var(--color-text-muted)">
                        "offset only = 5px"
                    </div>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Collision Detection"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "With avoidCollisions=true (default), the popper flips to the opposite side "
                "when there isn't enough space. Position the anchor near a viewport edge to test."
            </p>
            <div style="display: flex; justify-content: flex-start; padding: 2rem; background: var(--color-bg); border-radius: var(--radius); overflow: auto">
                <PopperRoot>
                    <PopperAnchor>
                        <button class="trigger-button">"Near Edge"</button>
                    </PopperAnchor>
                    <PopperContent
                        side=PopperSide::Left
                        side_offset=5
                        avoid_collisions=true
                        class="popper-content"
                    >
                        "Flips if no space"
                        <PopperArrow class="popper-arrow" />
                    </PopperContent>
                </PopperRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"CSS Custom Properties"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Popper provides CSS custom properties on the content element for responsive sizing."
            </p>
            <div style="display: flex; justify-content: center; padding: 4rem 2rem; background: var(--color-bg); border-radius: var(--radius)">
                <PopperRoot>
                    <PopperAnchor>
                        <button class="trigger-button" style="width: 150px">
                            "150px Wide Anchor"
                        </button>
                    </PopperAnchor>
                    <PopperContent
                        side=PopperSide::Bottom
                        side_offset=5
                        class="popper-content"
                        style="width: var(--radix-popper-anchor-width); text-align: center"
                    >
                        "Matches anchor width!"
                        <PopperArrow class="popper-arrow" />
                    </PopperContent>
                </PopperRoot>
            </div>
            <div style="font-size: 0.75rem; margin-top: 1rem; font-family: monospace">
                "Available properties:"
                <ul style="margin-top: 0.5rem">
                    <li>"--radix-popper-anchor-width"</li>
                    <li>"--radix-popper-anchor-height"</li>
                </ul>
            </div>
        </div>

        <div class="example-section">
            <h2>"Arrow Positioning Detail"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "The PopperArrow automatically positions and rotates based on the placed side. "
                "It uses these transforms:"
            </p>
            <ul style="font-size: 0.875rem; font-family: monospace; line-height: 2">
                <li><strong>"top:"</strong>" translateY(100%)"</li>
                <li><strong>"right:"</strong>" translateY(50%) rotate(90deg) translateX(-50%)"</li>
                <li><strong>"bottom:"</strong>" rotate(180deg)"</li>
                <li><strong>"left:"</strong>" translateY(50%) rotate(-90deg) translateX(50%)"</li>
            </ul>
        </div>

        <div class="example-section">
            <h2>"Implementation Notes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <strong>"Strategy:"</strong>" Uses \"fixed\" positioning for better scroll behavior"
                </li>
                <li>
                    <strong>"Auto-update:"</strong>" Repositions automatically when anchor moves or viewport changes"
                </li>
                <li>
                    <strong>"data-side:"</strong>" Content gets data-side attribute with actual placed side (may differ from requested if flipped)"
                </li>
                <li>
                    <strong>"data-align:"</strong>" Content gets data-align attribute with actual alignment"
                </li>
            </ul>
        </div>

        <style>
            ".popper-content {
                background: var(--color-surface);
                border: 1px solid var(--color-border);
                border-radius: var(--radius);
                padding: 0.5rem 0.75rem;
                font-size: 0.875rem;
            }
            .popper-content-mini {
                background: var(--color-surface);
                border: 1px solid var(--color-border);
                border-radius: var(--radius);
                padding: 0.25rem 0.5rem;
                font-size: 0.75rem;
            }
            .popper-arrow {
                fill: var(--color-surface);
            }"
        </style>
    }
}

#[component]
fn PositionDemo(side: PopperSide, align: PopperAlign, label: &'static str) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: center; padding: 3rem 1rem; background: var(--color-bg); border-radius: var(--radius); margin-bottom: 0.5rem">
            <PopperRoot>
                <PopperAnchor>
                    <span style="padding: 0.25rem 0.5rem; background: var(--color-accent); border-radius: var(--radius); font-size: 0.75rem">
                        {label}
                    </span>
                </PopperAnchor>
                <PopperContent
                    side=side
                    align=align
                    side_offset=5
                    avoid_collisions=false
                    class="popper-content-mini"
                >
                    <PopperArrow class="popper-arrow" width=8 height=4 />
                </PopperContent>
            </PopperRoot>
        </div>
    }
}
