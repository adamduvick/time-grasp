use leptos::prelude::*;
use radix::{
    AspectRatio, Orientation, ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb,
    ScrollAreaViewport, ScrollType,
};

#[component]
pub fn Radix() -> impl IntoView {
    view! {
        <ScrollAreaExample />
        <AspectRatioExample />
    }
}

#[component]
pub fn AspectRatioExample() -> impl IntoView {
    view! {
        <h2>"Aspect Ratio"</h2>
        <h3>"16 / 9 Image"</h3>
        <div
            style:width="300px"
            style:border-radius="6px"
            style:overflow="hidden"
        >
            <AspectRatio ratio=(16.0 / 9.0)>
                <img
                    class="Image"
                    style="object-fit: cover; width: 100%; height: 100%;" // style via Image class
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                    alt="Landscape photograph by Tobias Tullius"
                />
            </AspectRatio>
        </div>
        <h3>"1 / 1 Image"</h3>
        <div
            style:width="300px"
            style:border-radius="6px"
            style:overflow="hidden"
        >
            <AspectRatio ratio=(1.0 / 1.0)>
                <img
                    class="Image"
                    style="object-fit: cover; width: 100%; height: 100%;" // style via Image class
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                    alt="Landscape photograph by Tobias Tullius"
                />
            </AspectRatio>
        </div>
    }
}

// Example CSS for styling ScrollArea (would typically go in a CSS file):
// [data-radix-scroll-area-viewport]::-webkit-scrollbar { display: none; }
// [data-radix-scroll-area-scrollbar] { padding: 2px; background: rgba(0,0,0,0.1); border-radius: 6px; }
// [data-radix-scroll-area-scrollbar][data-orientation="vertical"] { width: 10px; top: 0; right: 4px; bottom: 0; }
// [data-radix-scroll-area-scrollbar][data-orientation="horizontal"] { height: 10px; left: 0; right: 0; bottom: 4px; }
// [data-radix-scroll-area-thumb] { background: rgba(0,0,0,0.4); border-radius: 9999px; cursor: pointer; }

/// Styled vertical scrollbar for examples
#[component]
fn StyledVerticalScrollbar(#[prop(default = "0".to_string())] bottom: String) -> impl IntoView {
    view! {
        <div
            style:position="absolute"
            style:top="0"
            style:right="4px"
            style:bottom=bottom
            style:width="10px"
            style:padding="2px"
            style:background="rgba(0,0,0,0.1)"
            style:border-radius="6px"
        >
            <ScrollAreaScrollbar orientation=Orientation::Vertical force_mount=true>
                <ScrollAreaThumb />
            </ScrollAreaScrollbar>
        </div>
    }
}

/// Styled horizontal scrollbar for examples
#[component]
fn StyledHorizontalScrollbar(#[prop(default = "0".to_string())] right: String) -> impl IntoView {
    view! {
        <div
            style:position="absolute"
            style:left="0"
            style:right=right
            style:bottom="4px"
            style:height="10px"
            style:padding="2px"
            style:background="rgba(0,0,0,0.1)"
            style:border-radius="6px"
        >
            <ScrollAreaScrollbar orientation=Orientation::Horizontal force_mount=true>
                <ScrollAreaThumb />
            </ScrollAreaScrollbar>
        </div>
    }
}

#[component]
pub fn ScrollAreaExample() -> impl IntoView {
    view! {
        <style inner_html="
            [data-radix-scroll-area-viewport]::-webkit-scrollbar { display: none; }
            [data-radix-scroll-area-scrollbar] { width: 100%; height: 100%; }
            [data-radix-scroll-area-thumb] { background: rgba(0,0,0,0.4); border-radius: 9999px; cursor: pointer; }
        " />

        <h2>"Scroll Area"</h2>
        <h3>"Vertical Scroll (Always Visible)"</h3>
        <div
            style:width="300px"
            style:height="200px"
            style:border="1px solid #ccc"
            style:border-radius="6px"
        >
            <ScrollAreaRoot scroll_type=ScrollType::Always>
                <ScrollAreaViewport>
                    <div style:padding="16px">
                        {(1..=50)
                            .map(|i| {
                                view! { <p>{format!("Line {i}: Lorem ipsum dolor sit amet")}</p> }
                            })
                            .collect_view()}
                    </div>
                </ScrollAreaViewport>
                <StyledVerticalScrollbar />
            </ScrollAreaRoot>
        </div>

        <h3>"Vertical Scroll (Hover)"</h3>
        <div
            style:width="300px"
            style:height="200px"
            style:border="1px solid #ccc"
            style:border-radius="6px"
        >
            <ScrollAreaRoot scroll_type=ScrollType::Hover>
                <ScrollAreaViewport>
                    <div style:padding="16px">
                        {(1..=50)
                            .map(|i| {
                                view! { <p>{format!("Line {i}: Lorem ipsum dolor sit amet")}</p> }
                            })
                            .collect_view()}
                    </div>
                </ScrollAreaViewport>
                <StyledVerticalScrollbar />
            </ScrollAreaRoot>
        </div>

        <h3>"Horizontal Scroll"</h3>
        <div
            style:width="300px"
            style:height="100px"
            style:border="1px solid #ccc"
            style:border-radius="6px"
        >
            <ScrollAreaRoot scroll_type=ScrollType::Hover>
                <ScrollAreaViewport>
                    <div style:white-space="nowrap" style:padding="16px">
                        {(1..=20)
                            .map(|i| {
                                view! {
                                    <span style:display="inline-block" style:margin-right="16px">
                                        {format!("Item {i}")}
                                    </span>
                                }
                            })
                            .collect_view()}
                    </div>
                </ScrollAreaViewport>
                <StyledHorizontalScrollbar />
            </ScrollAreaRoot>
        </div>

        <h3>"Both Scrollbars"</h3>
        <div
            style:width="300px"
            style:height="200px"
            style:border="1px solid #ccc"
            style:border-radius="6px"
        >
            <ScrollAreaRoot scroll_type=ScrollType::Always>
                <ScrollAreaViewport>
                    <div style:width="600px" style:padding="16px">
                        {(1..=50)
                            .map(|i| {
                                view! {
                                    <p style:white-space="nowrap">
                                        {format!(
                                            "Line {i}: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt",
                                        )}
                                    </p>
                                }
                            })
                            .collect_view()}
                    </div>
                </ScrollAreaViewport>
                <StyledVerticalScrollbar bottom="14px".to_string() />
                <StyledHorizontalScrollbar right="14px".to_string() />
            </ScrollAreaRoot>
        </div>
    }
}
