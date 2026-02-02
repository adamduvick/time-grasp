use leptos::prelude::*;
use radix::{ProgressIndicator, ProgressRoot};

#[component]
pub fn ProgressExample() -> impl IntoView {
    // Controlled progress for interactive example
    let progress1 = RwSignal::new(66.0_f64);
    let progress2 = RwSignal::new(33.0_f64);

    view! {
        <h1>"Progress"</h1>
        <p>
            "Progress indicator. Radix handles ARIA and data attributes. User styles "
            "the track, indicator, and any animations."
        </p>

        // Determinate Progress
        <div class="example-section">
            <h2>"Determinate Progress"</h2>
            <ProgressRoot
                class="progress-root"
                value=Signal::derive(move || Some(progress1.get()))
            >
                <ProgressIndicator
                    class="progress-indicator"
                    style=Signal::derive(move || format!("transform: translateX(-{}%)", 100.0 - progress1.get()))
                />
            </ProgressRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                {move || format!("{}% complete", progress1.get() as i32)}
            </p>
            <div style="margin-top: 1rem; display: flex; gap: 0.5rem">
                <button
                    class="trigger-button"
                    on:click=move |_| progress1.update(|v| *v = (*v - 10.0).max(0.0))
                >
                    "-10%"
                </button>
                <button
                    class="trigger-button"
                    on:click=move |_| progress1.update(|v| *v = (*v + 10.0).min(100.0))
                >
                    "+10%"
                </button>
                <button
                    class="trigger-button"
                    on:click=move |_| progress1.set(100.0)
                >
                    "Complete"
                </button>
            </div>
        </div>

        // Multiple Progress Bars
        <div class="example-section">
            <h2>"Multiple Progress Bars"</h2>
            <div style="display: flex; flex-direction: column; gap: 1rem">
                <div>
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem">
                        <span style="font-size: 0.875rem">"Uploading files..."</span>
                        <span style="font-size: 0.875rem">{move || format!("{}%", progress2.get() as i32)}</span>
                    </div>
                    <ProgressRoot
                        class="progress-root"
                        value=Signal::derive(move || Some(progress2.get()))
                    >
                        <ProgressIndicator
                            class="progress-indicator"
                            style=Signal::derive(move || format!("transform: translateX(-{}%)", 100.0 - progress2.get()))
                        />
                    </ProgressRoot>
                </div>

                <div>
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem">
                        <span style="font-size: 0.875rem">"Storage used"</span>
                        <span style="font-size: 0.875rem">"75%"</span>
                    </div>
                    <ProgressRoot
                        class="progress-root"
                        value=Signal::derive(|| Some(75.0))
                    >
                        <ProgressIndicator
                            class="progress-indicator"
                            style="transform: translateX(-25%); background: #f59e0b"
                        />
                    </ProgressRoot>
                </div>

                <div>
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem">
                        <span style="font-size: 0.875rem">"Memory"</span>
                        <span style="font-size: 0.875rem">"92%"</span>
                    </div>
                    <ProgressRoot
                        class="progress-root"
                        value=Signal::derive(|| Some(92.0))
                    >
                        <ProgressIndicator
                            class="progress-indicator"
                            style="transform: translateX(-8%); background: #dc2626"
                        />
                    </ProgressRoot>
                </div>
            </div>
            <div style="margin-top: 1rem">
                <button
                    class="trigger-button"
                    on:click=move |_| progress2.update(|v| *v = (*v + 15.0).min(100.0))
                >
                    "Simulate Upload Progress"
                </button>
            </div>
        </div>

        // Indeterminate
        <div class="example-section">
            <h2>"Indeterminate (unknown progress)"</h2>
            <ProgressRoot class="progress-root">
                <ProgressIndicator class="progress-indicator progress-indeterminate" />
            </ProgressRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "When value is None, data-state=\"indeterminate\". Animation is user-provided CSS."
            </p>
        </div>

        // Custom Max Value
        <div class="example-section">
            <h2>"Custom Max Value"</h2>
            <ProgressRoot
                class="progress-root"
                value=Signal::derive(|| Some(3.0))
                max=5.0
            >
                <ProgressIndicator
                    class="progress-indicator"
                    style="transform: translateX(-40%)"
                />
            </ProgressRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "3 of 5 steps complete (max=5)"
            </p>
        </div>

        // Data Attributes
        <div class="example-section">
            <h2>"Data Attributes"</h2>
            <p style="font-size: 0.875rem">
                "Radix sets these attributes on the Root element:"
            </p>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"data-state"</code>" - \"loading\" | \"complete\" | \"indeterminate\""
                </li>
                <li>
                    <code>"data-value"</code>" - current numeric value (when determinate)"
                </li>
                <li>
                    <code>"data-max"</code>" - maximum value (default 100)"
                </li>
            </ul>
            <p style="font-size: 0.875rem; margin-top: 1rem">
                "ARIA attributes are automatically set:"
            </p>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"role=\"progressbar\""</code>
                </li>
                <li>
                    <code>"aria-valuenow"</code>" - current value"
                </li>
                <li>
                    <code>"aria-valuemin=\"0\""</code>
                </li>
                <li>
                    <code>"aria-valuemax"</code>" - maximum value"
                </li>
            </ul>
        </div>
    }
}
