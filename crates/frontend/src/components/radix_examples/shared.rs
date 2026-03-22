use leptos::prelude::*;
use cardo_ui::{Orientation, ScrollAreaScrollbar, ScrollAreaThumb};

/// Styled vertical scrollbar for examples
#[component]
pub fn StyledVerticalScrollbar(#[prop(default = "0".to_string())] bottom: String) -> impl IntoView {
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
pub fn StyledHorizontalScrollbar(#[prop(default = "0".to_string())] right: String) -> impl IntoView {
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
