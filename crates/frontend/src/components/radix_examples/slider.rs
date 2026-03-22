use leptos::prelude::*;
use cardo_ui::{SliderOrientation, SliderRange, SliderRoot, SliderThumb, SliderTrack};

#[component]
pub fn SliderExample() -> impl IntoView {
    let basic_value = RwSignal::new(50.0);
    let vertical_value = RwSignal::new(30.0);
    let disabled_value = RwSignal::new(70.0);
    let custom_value = RwSignal::new(25.0);
    let step_value = RwSignal::new(0.0);

    view! {
        <h2>"Slider Primitive"</h2>

        <section style:margin-bottom="32px">
            <h3>"Basic Horizontal Slider"</h3>
            <p>"Current value: " {move || format!("{:.0}", basic_value.get())}</p>

            <div style:width="300px" style:padding="10px 0">
                <SliderRoot value=basic_value>
                    <SliderTrack>
                        <SliderRange />
                    </SliderTrack>
                    <SliderThumb />
                </SliderRoot>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Vertical Slider"</h3>
            <p>"Current value: " {move || format!("{:.0}", vertical_value.get())}</p>

            <div style:height="150px" style:width="40px" style:padding="10px">
                <SliderRoot
                    value=vertical_value
                    orientation=SliderOrientation::Vertical
                >
                    <SliderTrack>
                        <SliderRange />
                    </SliderTrack>
                    <SliderThumb />
                </SliderRoot>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Disabled Slider"</h3>
            <p>"Value (fixed): " {move || format!("{:.0}", disabled_value.get())}</p>

            <div style:width="300px" style:padding="10px 0" style:opacity="0.5">
                <SliderRoot value=disabled_value disabled=true>
                    <SliderTrack>
                        <SliderRange />
                    </SliderTrack>
                    <SliderThumb />
                </SliderRoot>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Custom Range (0-10, step 0.5)"</h3>
            <p>"Current value: " {move || format!("{:.1}", custom_value.get())}</p>

            <div style:width="300px" style:padding="10px 0">
                <SliderRoot
                    value=custom_value
                    min=0.0
                    max=10.0
                    step=0.5
                >
                    <SliderTrack>
                        <SliderRange />
                    </SliderTrack>
                    <SliderThumb />
                </SliderRoot>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Step Slider (-50 to 50, step 10)"</h3>
            <p>"Current value: " {move || format!("{:.0}", step_value.get())}</p>

            <div style:width="300px" style:padding="10px 0">
                <SliderRoot
                    value=step_value
                    min=-50.0
                    max=50.0
                    step=10.0
                >
                    <SliderTrack>
                        <SliderRange />
                    </SliderTrack>
                    <SliderThumb />
                </SliderRoot>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Keyboard Navigation"</h3>
            <ul>
                <li>"Arrow Left/Right or Up/Down: Adjust by step"</li>
                <li>"Page Up/Down: Adjust by 10x step"</li>
                <li>"Home: Jump to minimum"</li>
                <li>"End: Jump to maximum"</li>
            </ul>
        </section>
    }
}
