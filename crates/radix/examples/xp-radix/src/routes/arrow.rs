use leptos::prelude::*;
use radix::Arrow;

/// Arrow Primitive
///
/// A low-level utility primitive that renders an SVG arrow shape.
/// This is NOT an accessibility primitive - it's a composition helper
/// used internally by Tooltip, Popover, HoverCard, etc.
///
/// RADIX PROVIDES:
/// - Pre-configured SVG with proper viewBox and preserveAspectRatio
/// - Default triangle shape pointing down (tip at bottom)
/// - Customizable width and height
///
/// KEY DETAILS:
/// - viewBox is always "0 0 30 10" regardless of width/height
/// - preserveAspectRatio="none" allows independent scaling
/// - Default polygon: "0,0 30,0 15,10" (flat top, tip at bottom center)
/// - The arrow points DOWN by default - rotate as needed for other sides

#[component]
pub fn ArrowExample() -> impl IntoView {
    view! {
        <h1>"Arrow"</h1>
        <p>
            "Low-level SVG arrow primitive used for composition. The arrow always "
            "points down by default - rotate it for other orientations."
        </p>

        <div class="example-section">
            <h2>"Default Arrow"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Default size is width=10, height=5. The arrow points down."
            </p>
            <div style="display: flex; gap: 2rem; align-items: center">
                <div style="text-align: center">
                    <Arrow style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">
                        "Default (10x5)"
                    </div>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Different Sizes"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Width and height can be customized independently. The viewBox stays "
                "\"0 0 30 10\" but the SVG scales."
            </p>
            <div style="display: flex; gap: 2rem; align-items: flex-end; flex-wrap: wrap">
                <div style="text-align: center">
                    <Arrow width=10 height=5 style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">"10x5"</div>
                </div>
                <div style="text-align: center">
                    <Arrow width=20 height=10 style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">"20x10"</div>
                </div>
                <div style="text-align: center">
                    <Arrow width=30 height=15 style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">"30x15"</div>
                </div>
                <div style="text-align: center">
                    <Arrow width=20 height=5 style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">"20x5 (wide)"</div>
                </div>
                <div style="text-align: center">
                    <Arrow width=10 height=15 style="fill: var(--color-text)" />
                    <div style="font-size: 0.75rem; margin-top: 0.5rem">"10x15 (tall)"</div>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Rotations for Different Sides"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "The arrow points down by default. Rotate for other orientations."
            </p>
            <div style="display: flex; gap: 3rem; align-items: center; flex-wrap: wrap">
                // Side: top (arrow points down, no rotation)
                <div style="text-align: center">
                    <div style="background: var(--color-surface); padding: 0.5rem 1rem; border-radius: var(--radius); position: relative; margin-bottom: 5px">
                        "Content above"
                        <Arrow
                            width=10
                            height=5
                            style="fill: var(--color-surface); position: absolute; bottom: 0; left: 50%; transform: translateX(-50%) translateY(100%)"
                        />
                    </div>
                    <div style="font-size: 0.75rem; color: var(--color-text-muted)">
                        "Side: top (no rotation)"
                    </div>
                </div>

                // Side: bottom (arrow points up, rotate 180deg)
                <div style="text-align: center">
                    <div style="background: var(--color-surface); padding: 0.5rem 1rem; border-radius: var(--radius); position: relative; margin-top: 5px">
                        "Content below"
                        <Arrow
                            width=10
                            height=5
                            style="fill: var(--color-surface); position: absolute; top: 0; left: 50%; transform: translateX(-50%) translateY(-100%) rotate(180deg)"
                        />
                    </div>
                    <div style="font-size: 0.75rem; color: var(--color-text-muted)">
                        "Side: bottom (rotate 180deg)"
                    </div>
                </div>

                // Side: left (arrow points right, rotate -90deg)
                <div style="text-align: center; display: flex; align-items: center; gap: 5px">
                    <div style="background: var(--color-surface); padding: 0.5rem 1rem; border-radius: var(--radius); position: relative">
                        "Content left"
                        <Arrow
                            width=10
                            height=5
                            style="fill: var(--color-surface); position: absolute; right: 0; top: 50%; transform: translateY(-50%) translateX(100%) rotate(-90deg)"
                        />
                    </div>
                    <div style="font-size: 0.75rem; color: var(--color-text-muted)">
                        "Side: left"<br />"(rotate -90deg)"
                    </div>
                </div>

                // Side: right (arrow points left, rotate 90deg)
                <div style="text-align: center; display: flex; align-items: center; gap: 5px">
                    <div style="font-size: 0.75rem; color: var(--color-text-muted)">
                        "Side: right"<br />"(rotate 90deg)"
                    </div>
                    <div style="background: var(--color-surface); padding: 0.5rem 1rem; border-radius: var(--radius); position: relative">
                        "Content right"
                        <Arrow
                            width=10
                            height=5
                            style="fill: var(--color-surface); position: absolute; left: 0; top: 50%; transform: translateY(-50%) translateX(-100%) rotate(90deg)"
                        />
                    </div>
                </div>
            </div>
        </div>

        <div class="example-section">
            <h2>"Custom Colors"</h2>
            <div style="display: flex; gap: 2rem; align-items: center">
                <Arrow width=20 height=10 style="fill: var(--color-accent)" />
                <Arrow width=20 height=10 style="fill: #22c55e" />
                <Arrow width=20 height=10 style="fill: #ef4444" />
                <Arrow width=20 height=10 style="fill: #f59e0b" />
            </div>
        </div>

        <div class="example-section">
            <h2>"With Border (stroke)"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "You can add a stroke for bordered arrows, but note this adds stroke on all sides."
            </p>
            <div style="display: flex; gap: 2rem; align-items: center">
                <Arrow
                    width=20
                    height=10
                    style="fill: var(--color-surface); stroke: var(--color-border); stroke-width: 1"
                />
            </div>
        </div>

        <div class="example-section">
            <h2>"Implementation Notes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <strong>"viewBox:"</strong>" Always \"0 0 30 10\" - the polygon is drawn in this coordinate space"
                </li>
                <li>
                    <strong>"preserveAspectRatio:"</strong>" Set to \"none\" so width/height scale independently"
                </li>
                <li>
                    <strong>"Default polygon:"</strong>" \"0,0 30,0 15,10\" - flat top edge, tip at bottom center"
                </li>
                <li>
                    <strong>"Usage:"</strong>" Typically used inside Popper, Tooltip, Popover primitives"
                </li>
            </ul>
        </div>
    }
}
