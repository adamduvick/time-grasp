use leptos::prelude::*;
use cardo_ui::AspectRatio;

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
                    style="object-fit: cover; width: 100%; height: 100%;"
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
                    style="object-fit: cover; width: 100%; height: 100%;"
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                    alt="Landscape photograph by Tobias Tullius"
                />
            </AspectRatio>
        </div>
    }
}
