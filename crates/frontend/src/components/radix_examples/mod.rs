use leptos::prelude::*;
use radix::AspectRatio;

#[component]
pub fn Radix() -> impl IntoView {
    view! {
        <h2>"16 / 9 Image"</h2>
        <AspectRatio ratio=(16.0 / 9.0)>
            <img
                class="Image"
                style="object-fit: cover; width: 100%; height: 100%;" // style via Image class
                src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                alt="Landscape photograph by Tobias Tullius"
            />
        </AspectRatio>
        <h2>"9 / 16 Image"</h2>
        <AspectRatio ratio=(9.0 / 16.0)>
            <img
                class="Image"
                style="object-fit: cover; width: 100%; height: 100%;" // style via Image class
                src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
                alt="Landscape photograph by Tobias Tullius"
            />
        </AspectRatio>
    }
}
