use leptos::prelude::*;
use radix::{ScrollAreaRoot, ScrollAreaViewport, ScrollType};

use super::shared::{StyledHorizontalScrollbar, StyledVerticalScrollbar};

#[component]
pub fn ScrollAreaExample() -> impl IntoView {
    view! {
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
