use leptos::prelude::*;
use radix::{SliderOrientation, SliderRange, SliderRoot, SliderThumb, SliderTrack};

#[component]
pub fn SliderExample() -> impl IntoView {
    // Controlled state for various examples
    let basic = RwSignal::new(vec![50.0]);
    let controlled = RwSignal::new(vec![50.0]);
    let range = RwSignal::new(vec![25.0, 75.0]);
    let step10 = RwSignal::new(vec![50.0]);
    let step25 = RwSignal::new(vec![50.0]);
    let volume = RwSignal::new(vec![80.0]);
    let vertical1 = RwSignal::new(vec![50.0]);
    let vertical2 = RwSignal::new(vec![30.0]);
    let vertical3 = RwSignal::new(vec![70.0]);
    let disabled = RwSignal::new(vec![50.0]);
    let year = RwSignal::new(vec![2020.0]);

    view! {
        <h1>"Slider"</h1>
        <p>
            "Range input with single or multiple thumbs. Radix handles keyboard "
            "control, ARIA, and step snapping. User styles track, range, and thumb."
        </p>

        // Basic Slider
        <div class="example-section">
            <h2>"Basic Slider"</h2>
            <SliderRoot class="slider-root" values=basic>
                <SliderTrack class="slider-track">
                    <SliderRange class="slider-range" />
                </SliderTrack>
                <SliderThumb class="slider-thumb" />
            </SliderRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys (\u{00B1}1), Shift+Arrow (\u{00B1}10), Home/End"
            </p>
        </div>

        // Controlled Value
        <div class="example-section">
            <h2>"Controlled Value"</h2>
            <SliderRoot class="slider-root" values=controlled>
                <SliderTrack class="slider-track">
                    <SliderRange class="slider-range" />
                </SliderTrack>
                <SliderThumb class="slider-thumb" />
            </SliderRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "Value: "<strong>{move || controlled.get().first().map(|v| *v as i32).unwrap_or(0)}</strong>
            </p>
            <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem">
                <button class="trigger-button" on:click=move |_| controlled.set(vec![0.0])>
                    "Min"
                </button>
                <button class="trigger-button" on:click=move |_| controlled.set(vec![50.0])>
                    "50"
                </button>
                <button class="trigger-button" on:click=move |_| controlled.set(vec![100.0])>
                    "Max"
                </button>
            </div>
        </div>

        // Range Slider (Two Thumbs)
        <div class="example-section">
            <h2>"Range Slider (Two Thumbs)"</h2>
            <SliderRoot class="slider-root" values=range min_steps_between_thumbs=1>
                <SliderTrack class="slider-track">
                    <SliderRange class="slider-range" />
                </SliderTrack>
                <SliderThumb class="slider-thumb" />
                <SliderThumb class="slider-thumb" />
            </SliderRoot>
            <p style="margin-top: 0.5rem; font-size: 0.875rem">
                "Range: "<strong>{move || range.get().first().map(|v| *v as i32).unwrap_or(0)}</strong>
                " - "<strong>{move || range.get().get(1).map(|v| *v as i32).unwrap_or(0)}</strong>
            </p>
        </div>

        // With Step
        <div class="example-section">
            <h2>"With Step"</h2>
            <div style="display: flex; flex-direction: column; gap: 1.5rem">
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "Step: 10"
                    </p>
                    <SliderRoot class="slider-root" values=step10 step=10.0>
                        <SliderTrack class="slider-track">
                            <SliderRange class="slider-range" />
                        </SliderTrack>
                        <SliderThumb class="slider-thumb" />
                    </SliderRoot>
                </div>
                <div>
                    <p style="font-size: 0.875rem; margin-bottom: 0.5rem">
                        "Step: 25"
                    </p>
                    <SliderRoot class="slider-root" values=step25 step=25.0>
                        <SliderTrack class="slider-track">
                            <SliderRange class="slider-range" />
                        </SliderTrack>
                        <SliderThumb class="slider-thumb" />
                    </SliderRoot>
                </div>
            </div>
        </div>

        // Volume Control Example
        <div class="example-section">
            <h2>"Volume Control Example"</h2>
            <div style="display: flex; align-items: center; gap: 1rem">
                <span>"Vol"</span>
                <SliderRoot class="slider-root" values=volume style="flex: 1">
                    <SliderTrack class="slider-track">
                        <SliderRange class="slider-range" />
                    </SliderTrack>
                    <SliderThumb class="slider-thumb" />
                </SliderRoot>
                <span style="width: 3rem; text-align: right">{move || format!("{}%", volume.get().first().map(|v| *v as i32).unwrap_or(0))}</span>
            </div>
        </div>

        // Vertical Orientation
        <div class="example-section">
            <h2>"Vertical Orientation"</h2>
            <div style="display: flex; gap: 2rem; height: 150px">
                <SliderRoot
                    class="slider-root"
                    values=vertical1
                    orientation=SliderOrientation::Vertical
                >
                    <SliderTrack class="slider-track">
                        <SliderRange class="slider-range" />
                    </SliderTrack>
                    <SliderThumb class="slider-thumb" />
                </SliderRoot>
                <SliderRoot
                    class="slider-root"
                    values=vertical2
                    orientation=SliderOrientation::Vertical
                >
                    <SliderTrack class="slider-track">
                        <SliderRange class="slider-range" />
                    </SliderTrack>
                    <SliderThumb class="slider-thumb" />
                </SliderRoot>
                <SliderRoot
                    class="slider-root"
                    values=vertical3
                    orientation=SliderOrientation::Vertical
                >
                    <SliderTrack class="slider-track">
                        <SliderRange class="slider-range" />
                    </SliderTrack>
                    <SliderThumb class="slider-thumb" />
                </SliderRoot>
            </div>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Up/Down arrows for vertical sliders"
            </p>
        </div>

        // Disabled Slider
        <div class="example-section">
            <h2>"Disabled Slider"</h2>
            <SliderRoot class="slider-root" values=disabled disabled=true>
                <SliderTrack class="slider-track">
                    <SliderRange class="slider-range" />
                </SliderTrack>
                <SliderThumb class="slider-thumb" />
            </SliderRoot>
        </div>

        // Custom Min/Max
        <div class="example-section">
            <h2>"Custom Min/Max"</h2>
            <SliderRoot class="slider-root" values=year min=2000.0 max=2030.0>
                <SliderTrack class="slider-track">
                    <SliderRange class="slider-range" />
                </SliderTrack>
                <SliderThumb class="slider-thumb" />
            </SliderRoot>
            <div style="display: flex; justify-content: space-between; font-size: 0.75rem; color: var(--color-text-muted); margin-top: 0.25rem">
                <span>"2000"</span>
                <span>{move || year.get().first().map(|v| *v as i32).unwrap_or(0)}</span>
                <span>"2030"</span>
            </div>
        </div>

        // Data Attributes
        <div class="example-section">
            <h2>"Data Attributes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"data-orientation"</code>" - \"horizontal\" | \"vertical\""
                </li>
                <li>
                    <code>"data-disabled"</code>" - Present when disabled"
                </li>
                <li>
                    <code>"data-state"</code>" on Thumb: \"idle\" | \"dragging\""
                </li>
                <li>
                    <code>"role=\"slider\""</code>" with aria-valuemin, aria-valuemax, aria-valuenow"
                </li>
            </ul>
        </div>
    }
}
