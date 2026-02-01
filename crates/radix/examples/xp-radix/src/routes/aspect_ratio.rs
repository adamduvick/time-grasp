use leptos::prelude::*;
use radix::AspectRatio;

#[component]
pub fn AspectRatioExample() -> impl IntoView {
    view! {
        <h1>"AspectRatio"</h1>
        <p>
            "Maintains consistent width/height ratio for content. Useful for images, "
            "videos, and responsive containers."
        </p>

        // 16:9 Ratio (Video)
        <div class="example-section">
            <h2>"16:9 Ratio (Video)"</h2>
            <div style="width: 100%; max-width: 400px">
                <AspectRatio ratio={16.0 / 9.0}>
                    <div style="width: 100%; height: 100%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; color: white; font-size: 1.5rem; border-radius: var(--radius)">
                        "16:9"
                    </div>
                </AspectRatio>
            </div>
        </div>

        // 1:1 Ratio (Square)
        <div class="example-section">
            <h2>"1:1 Ratio (Square)"</h2>
            <div style="width: 150px">
                <AspectRatio ratio=1.0>
                    <div style="width: 100%; height: 100%; background: var(--color-accent); display: flex; align-items: center; justify-content: center; color: white; border-radius: var(--radius)">
                        "1:1"
                    </div>
                </AspectRatio>
            </div>
        </div>

        // 4:3 Ratio (Classic Photo)
        <div class="example-section">
            <h2>"4:3 Ratio (Classic Photo)"</h2>
            <div style="width: 200px">
                <AspectRatio ratio={4.0 / 3.0}>
                    <img
                        src="https://picsum.photos/400/300"
                        alt="Random landscape"
                        style="width: 100%; height: 100%; object-fit: cover; border-radius: var(--radius)"
                    />
                </AspectRatio>
            </div>
        </div>

        // Different Ratios Comparison
        <div class="example-section">
            <h2>"Different Ratios Comparison"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap">
                <div style="width: 150px">
                    <AspectRatio ratio=1.0>
                        <div style="width: 100%; height: 100%; background: var(--color-surface); border: 1px solid var(--color-border); display: flex; align-items: center; justify-content: center; border-radius: var(--radius)">
                            "1:1"
                        </div>
                    </AspectRatio>
                </div>
                <div style="width: 150px">
                    <AspectRatio ratio={4.0 / 3.0}>
                        <div style="width: 100%; height: 100%; background: var(--color-surface); border: 1px solid var(--color-border); display: flex; align-items: center; justify-content: center; border-radius: var(--radius)">
                            "4:3"
                        </div>
                    </AspectRatio>
                </div>
                <div style="width: 150px">
                    <AspectRatio ratio={16.0 / 9.0}>
                        <div style="width: 100%; height: 100%; background: var(--color-surface); border: 1px solid var(--color-border); display: flex; align-items: center; justify-content: center; border-radius: var(--radius)">
                            "16:9"
                        </div>
                    </AspectRatio>
                </div>
                <div style="width: 150px">
                    <AspectRatio ratio={21.0 / 9.0}>
                        <div style="width: 100%; height: 100%; background: var(--color-surface); border: 1px solid var(--color-border); display: flex; align-items: center; justify-content: center; border-radius: var(--radius)">
                            "21:9"
                        </div>
                    </AspectRatio>
                </div>
            </div>
        </div>

        // Portrait Ratio (2:3)
        <div class="example-section">
            <h2>"Portrait Ratio (2:3)"</h2>
            <div style="width: 150px">
                <AspectRatio ratio={2.0 / 3.0}>
                    <div style="width: 100%; height: 100%; background: linear-gradient(180deg, #f093fb 0%, #f5576c 100%); display: flex; align-items: center; justify-content: center; color: white; border-radius: var(--radius)">
                        "2:3"
                    </div>
                </AspectRatio>
            </div>
        </div>

        // Responsive Container
        <div class="example-section">
            <h2>"Responsive Container"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Resize the browser to see the aspect ratio maintained."
            </p>
            <div style="width: 100%; max-width: 500px">
                <AspectRatio ratio={16.0 / 9.0}>
                    <div style="width: 100%; height: 100%; background: var(--color-border); display: flex; align-items: center; justify-content: center; border-radius: var(--radius)">
                        "Responsive 16:9 container"
                    </div>
                </AspectRatio>
            </div>
        </div>

        // What Radix Provides
        <div class="example-section">
            <h2>"What Radix Provides"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>"CSS aspect-ratio property with padding-bottom fallback"</li>
                <li>"Content automatically fills the container"</li>
                <li>"Works with any child content"</li>
            </ul>
            <p style="font-size: 0.875rem; margin-top: 1rem">
                "This is purely a layout utility - no ARIA or interactivity involved."
            </p>
        </div>
    }
}
