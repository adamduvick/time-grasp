use leptos::prelude::*;
use radix::{
    Orientation, ScrollAreaCorner, ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb,
    ScrollAreaViewport, ScrollType,
};

#[component]
pub fn ScrollAreaExample() -> impl IntoView {
    view! {
        <h1>"ScrollArea"</h1>
        <p>
            "Custom scrollbars with consistent cross-browser styling. Radix handles "
            "scrollbar behavior and touch support. User styles the scrollbar parts."
        </p>

        // Vertical Scroll
        <div class="example-section">
            <h2>"Vertical Scroll"</h2>
            <ScrollAreaRoot class="scrollarea-root" scroll_type=ScrollType::Hover style="height: 200px">
                <ScrollAreaViewport class="scrollarea-viewport">
                    <div style="padding: 1rem">
                        <h3 style="margin-bottom: 0.5rem">"Tags"</h3>
                        {(1..=50).map(|i| {
                            view! {
                                <div style="padding: 0.5rem; border-bottom: 1px solid var(--color-border)">
                                    {format!("Item {}", i)}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </ScrollAreaViewport>
                <ScrollAreaScrollbar
                    class="scrollarea-scrollbar"
                    orientation=Orientation::Vertical
                >
                    <ScrollAreaThumb class="scrollarea-thumb" />
                </ScrollAreaScrollbar>
            </ScrollAreaRoot>
        </div>

        // Horizontal Scroll
        <div class="example-section">
            <h2>"Horizontal Scroll"</h2>
            <ScrollAreaRoot class="scrollarea-root" scroll_type=ScrollType::Hover style="width: 100%">
                <ScrollAreaViewport class="scrollarea-viewport">
                    <div style="display: flex; gap: 1rem; padding: 1rem; width: max-content">
                        {(1..=20).map(|i| {
                            view! {
                                <div style="width: 100px; height: 100px; background: var(--color-accent); border-radius: var(--radius); display: flex; align-items: center; justify-content: center; color: white; flex-shrink: 0">
                                    {i}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </ScrollAreaViewport>
                <ScrollAreaScrollbar
                    class="scrollarea-scrollbar"
                    orientation=Orientation::Horizontal
                >
                    <ScrollAreaThumb class="scrollarea-thumb" />
                </ScrollAreaScrollbar>
            </ScrollAreaRoot>
        </div>

        // Both Scrollbars
        <div class="example-section">
            <h2>"Both Scrollbars"</h2>
            <ScrollAreaRoot class="scrollarea-root" scroll_type=ScrollType::Hover style="height: 200px; width: 300px">
                <ScrollAreaViewport class="scrollarea-viewport">
                    <div style="width: 600px; padding: 1rem">
                        <h3>"Wide Content"</h3>
                        {(1..=30).map(|i| {
                            view! {
                                <p style="white-space: nowrap; margin: 0.5rem 0">
                                    {format!("This is line {} with some extra text to make it wider than the container allows.", i)}
                                </p>
                            }
                        }).collect_view()}
                    </div>
                </ScrollAreaViewport>
                <ScrollAreaScrollbar
                    class="scrollarea-scrollbar"
                    orientation=Orientation::Vertical
                >
                    <ScrollAreaThumb class="scrollarea-thumb" />
                </ScrollAreaScrollbar>
                <ScrollAreaScrollbar
                    class="scrollarea-scrollbar"
                    orientation=Orientation::Horizontal
                >
                    <ScrollAreaThumb class="scrollarea-thumb" />
                </ScrollAreaScrollbar>
                <ScrollAreaCorner class="scrollarea-corner" />
            </ScrollAreaRoot>
        </div>

        // Scrollbar Types
        <div class="example-section">
            <h2>"Scrollbar Types"</h2>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem">
                // type="auto" (default)
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "type=\"auto\" (default)"
                    </p>
                    <ScrollAreaRoot
                        class="scrollarea-root"
                        scroll_type=ScrollType::Auto
                        style="height: 150px"
                    >
                        <ScrollAreaViewport class="scrollarea-viewport">
                            <div style="padding: 1rem">
                                {(1..=20).map(|i| {
                                    view! {
                                        <div style="padding: 0.25rem 0">
                                            {format!("Item {}", i)}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </ScrollAreaViewport>
                        <ScrollAreaScrollbar
                            class="scrollarea-scrollbar"
                            orientation=Orientation::Vertical
                        >
                            <ScrollAreaThumb class="scrollarea-thumb" />
                        </ScrollAreaScrollbar>
                    </ScrollAreaRoot>
                </div>

                // type="always"
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "type=\"always\""
                    </p>
                    <ScrollAreaRoot
                        class="scrollarea-root"
                        scroll_type=ScrollType::Always
                        style="height: 150px"
                    >
                        <ScrollAreaViewport class="scrollarea-viewport">
                            <div style="padding: 1rem">
                                {(1..=20).map(|i| {
                                    view! {
                                        <div style="padding: 0.25rem 0">
                                            {format!("Item {}", i)}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </ScrollAreaViewport>
                        <ScrollAreaScrollbar
                            class="scrollarea-scrollbar"
                            orientation=Orientation::Vertical
                        >
                            <ScrollAreaThumb class="scrollarea-thumb" />
                        </ScrollAreaScrollbar>
                    </ScrollAreaRoot>
                </div>

                // type="hover"
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "type=\"hover\""
                    </p>
                    <ScrollAreaRoot
                        class="scrollarea-root"
                        scroll_type=ScrollType::Hover
                        style="height: 150px"
                    >
                        <ScrollAreaViewport class="scrollarea-viewport">
                            <div style="padding: 1rem">
                                {(1..=20).map(|i| {
                                    view! {
                                        <div style="padding: 0.25rem 0">
                                            {format!("Item {}", i)}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </ScrollAreaViewport>
                        <ScrollAreaScrollbar
                            class="scrollarea-scrollbar"
                            orientation=Orientation::Vertical
                        >
                            <ScrollAreaThumb class="scrollarea-thumb" />
                        </ScrollAreaScrollbar>
                    </ScrollAreaRoot>
                </div>

                // type="scroll"
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "type=\"scroll\""
                    </p>
                    <ScrollAreaRoot
                        class="scrollarea-root"
                        scroll_type=ScrollType::Scroll
                        style="height: 150px"
                    >
                        <ScrollAreaViewport class="scrollarea-viewport">
                            <div style="padding: 1rem">
                                {(1..=20).map(|i| {
                                    view! {
                                        <div style="padding: 0.25rem 0">
                                            {format!("Item {}", i)}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </ScrollAreaViewport>
                        <ScrollAreaScrollbar
                            class="scrollarea-scrollbar"
                            orientation=Orientation::Vertical
                        >
                            <ScrollAreaThumb class="scrollarea-thumb" />
                        </ScrollAreaScrollbar>
                    </ScrollAreaRoot>
                </div>
            </div>
        </div>

        // Scrollbar Types Explained
        <div class="example-section">
            <h2>"Scrollbar Types Explained"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"type=\"auto\""</code>" - Scrollbar visible when content overflows"
                </li>
                <li>
                    <code>"type=\"always\""</code>" - Scrollbar always visible"
                </li>
                <li>
                    <code>"type=\"hover\""</code>" - Scrollbar visible when hovering over scroll area"
                </li>
                <li>
                    <code>"type=\"scroll\""</code>" - Scrollbar visible when scrolling"
                </li>
            </ul>
        </div>
    }
}
