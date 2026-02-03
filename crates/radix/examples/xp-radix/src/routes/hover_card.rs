use leptos::prelude::*;
use radix::{
    HoverCardArrow, HoverCardContent, HoverCardPortal, HoverCardRoot, HoverCardSide,
    HoverCardTrigger,
};

/// HoverCard Primitive
///
/// RADIX PROVIDES:
/// - Open on hover with configurable delays (openDelay, closeDelay)
/// - Positioning relative to trigger
/// - Collision detection
/// - Arrow component
/// - data-state="open" | "closed"
/// - data-side and data-align
/// - Closes when pointer leaves both trigger and content
///
/// USER MUST IMPLEMENT:
/// - Card content styling
/// - Arrow styling
/// - Animations (using data-state)
///
/// DIFFERENT FROM TOOLTIP:
/// - HoverCard can contain interactive content
/// - Tooltip is for simple text hints
/// - HoverCard has longer default delays

#[component]
pub fn HoverCardExample() -> impl IntoView {
    view! {
        <h1>"HoverCard"</h1>
        <p>
            "Rich content on hover. Unlike Tooltip (simple hints), HoverCard can "
            "contain interactive content like links and buttons."
        </p>

        <div class="example-section">
            <h2>"Basic HoverCard"</h2>
            <HoverCardRoot>
                <HoverCardTrigger>
                    <a
                        href="https://twitter.com/radaborhani"
                        target="_blank"
                        rel="noreferrer"
                        style="color: var(--color-accent)"
                    >
                        "@radaborhani"
                    </a>
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent class="hovercard-content" side_offset=5>
                        <div style="display: flex; gap: 1rem">
                            <div style="width: 48px; height: 48px; border-radius: 50%; background: var(--color-accent); flex-shrink: 0" />
                            <div>
                                <h3 style="margin: 0">"Rad Aborhani"</h3>
                                <p style="margin: 0.25rem 0; color: var(--color-text-muted)">
                                    "@radaborhani"
                                </p>
                                <p style="margin: 0.5rem 0 0; font-size: 0.875rem">
                                    "Building things at Radix. Previously at somewhere else."
                                </p>
                            </div>
                        </div>
                        <HoverCardArrow class="hovercard-arrow" />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCardRoot>
            <span style="margin-left: 0.5rem">"- hover over the link"</span>
        </div>

        <div class="example-section">
            <h2>"With Interactive Content"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "HoverCard content can contain links and buttons (unlike Tooltip)."
            </p>
            <HoverCardRoot>
                <HoverCardTrigger>
                    <span style="cursor: pointer; border-bottom: 1px dashed var(--color-accent)">
                        "React"
                    </span>
                </HoverCardTrigger>
                <HoverCardPortal>
                    <HoverCardContent class="hovercard-content" side_offset=5>
                        <h3 style="margin: 0 0 0.5rem">"React"</h3>
                        <p style="margin: 0 0 0.5rem; font-size: 0.875rem">
                            "A JavaScript library for building user interfaces."
                        </p>
                        <div style="display: flex; gap: 0.5rem">
                            <a
                                href="https://react.dev"
                                target="_blank"
                                rel="noreferrer"
                                class="trigger-button"
                                style="font-size: 0.75rem; padding: 0.25rem 0.5rem"
                            >
                                "Documentation"
                            </a>
                            <a
                                href="https://github.com/facebook/react"
                                target="_blank"
                                rel="noreferrer"
                                class="trigger-button"
                                style="font-size: 0.75rem; padding: 0.25rem 0.5rem"
                            >
                                "GitHub"
                            </a>
                        </div>
                        <HoverCardArrow class="hovercard-arrow" />
                    </HoverCardContent>
                </HoverCardPortal>
            </HoverCardRoot>
        </div>

        <div class="example-section">
            <h2>"Custom Delays"</h2>
            <div style="display: flex; gap: 2rem">
                <HoverCardRoot open_delay=0 close_delay=0>
                    <HoverCardTrigger>
                        <button class="trigger-button">"Instant (0ms)"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side_offset=5>
                            "Opens and closes immediately"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>

                <HoverCardRoot open_delay=500 close_delay=300>
                    <HoverCardTrigger>
                        <button class="trigger-button">"Slow (500ms open)"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side_offset=5>
                            "Takes 500ms to open, 300ms to close"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"Different Positions"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap">
                <HoverCardRoot>
                    <HoverCardTrigger>
                        <button class="trigger-button">"top"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side=HoverCardSide::Top side_offset=5>
                            "Card positioned on top"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>

                <HoverCardRoot>
                    <HoverCardTrigger>
                        <button class="trigger-button">"right"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side=HoverCardSide::Right side_offset=5>
                            "Card positioned on right"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>

                <HoverCardRoot>
                    <HoverCardTrigger>
                        <button class="trigger-button">"bottom"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side=HoverCardSide::Bottom side_offset=5>
                            "Card positioned on bottom"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>

                <HoverCardRoot>
                    <HoverCardTrigger>
                        <button class="trigger-button">"left"</button>
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent class="hovercard-content" side=HoverCardSide::Left side_offset=5>
                            "Card positioned on left"
                            <HoverCardArrow class="hovercard-arrow" />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCardRoot>
            </div>
        </div>

        <div class="example-section">
            <h2>"HoverCard vs Tooltip"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Tooltip"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"HoverCard"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Content"</td>
                        <td style="padding: 0.5rem">"Simple text"</td>
                        <td style="padding: 0.5rem">"Rich/interactive"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Interactive content"</td>
                        <td style="padding: 0.5rem">"No"</td>
                        <td style="padding: 0.5rem">"Yes (links, buttons)"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Default delay"</td>
                        <td style="padding: 0.5rem">"Short (700ms)"</td>
                        <td style="padding: 0.5rem">"Longer"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Provider required"</td>
                        <td style="padding: 0.5rem">"Yes"</td>
                        <td style="padding: 0.5rem">"No"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
