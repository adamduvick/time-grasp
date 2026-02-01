mod aspect_ratio;
mod scroll_area;
mod separator;
mod shared;
mod slider;

use leptos::prelude::*;
use leptos_router::components::{A, Outlet, ParentRoute, Route};
use leptos_router::MatchNestedRoutes;
use leptos_router_macro::path;
use radix::{ScrollAreaRoot, ScrollAreaViewport, ScrollType};

use aspect_ratio::AspectRatioExample;
use scroll_area::ScrollAreaExample;
use separator::SeparatorExample;
use shared::StyledVerticalScrollbar;
use slider::SliderExample;

/// Radix examples routes - handles all subrouting for /radix/*
#[component(transparent)]
pub fn RadixRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("radix") view=RadixLayout>
            <Route path=path!("/") view=|| "Select a primitive." />
            <Route path=path!("/aspect-ratio") view=AspectRatioExample />
            <Route path=path!("/scroll-area") view=ScrollAreaExample />
            <Route path=path!("/separator") view=SeparatorExample />
            <Route path=path!("/slider") view=SliderExample />
        </ParentRoute>
    }
    .into_inner()
}

/// Layout component for the /radix route with sidebar navigation
#[component]
fn RadixLayout() -> impl IntoView {
    view! {
        <style inner_html="
            [data-radix-scroll-area-viewport]::-webkit-scrollbar { display: none; }
            [data-radix-scroll-area-scrollbar] { width: 100%; height: 100%; }
            [data-radix-scroll-area-thumb] { background: rgba(0,0,0,0.4); border-radius: 9999px; cursor: pointer; }
        " />

        <div style:display="flex" style:height="calc(100vh - 60px)" style:gap="16px">
            // Left sidebar (20%)
            <div
                style:width="20%"
                style:min-width="150px"
                style:border="1px solid #ccc"
                style:border-radius="6px"
            >
                <ScrollAreaRoot scroll_type=ScrollType::Auto>
                    <ScrollAreaViewport>
                        <nav style:padding="16px">
                            <h3 style:margin-top="0">"Primitives"</h3>
                            <ul style:list-style="none" style:padding="0" style:margin="0">
                                <li style:margin-bottom="8px">
                                    <A href="/radix/aspect-ratio">"Aspect Ratio"</A>
                                </li>
                                <li style:margin-bottom="8px">
                                    <A href="/radix/scroll-area">"Scroll Area"</A>
                                </li>
                                <li style:margin-bottom="8px">
                                    <A href="/radix/separator">"Separator"</A>
                                </li>
                                <li style:margin-bottom="8px">
                                    <A href="/radix/slider">"Slider"</A>
                                </li>
                            </ul>
                        </nav>
                    </ScrollAreaViewport>
                    <StyledVerticalScrollbar />
                </ScrollAreaRoot>
            </div>

            // Right content area (80%)
            <div
                style:width="80%"
                style:border="1px solid #ccc"
                style:border-radius="6px"
            >
                <ScrollAreaRoot scroll_type=ScrollType::Auto>
                    <ScrollAreaViewport>
                        <div style:padding="16px">
                            <Outlet />
                        </div>
                    </ScrollAreaViewport>
                    <StyledVerticalScrollbar />
                </ScrollAreaRoot>
            </div>
        </div>
    }
}
