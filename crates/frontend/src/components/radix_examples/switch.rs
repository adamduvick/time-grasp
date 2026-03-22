use leptos::prelude::*;
use cardo_ui::{SwitchRoot, SwitchThumb};

#[component]
pub fn SwitchExample() -> impl IntoView {
    let basic_checked = RwSignal::new(false);
    let default_on = RwSignal::new(true);
    let disabled_off = RwSignal::new(false);
    let disabled_on = RwSignal::new(true);
    let labeled_checked = RwSignal::new(false);

    view! {
        <h2>"Switch Primitive"</h2>

        <section style:margin-bottom="32px">
            <h3>"Basic Switch"</h3>
            <p>"State: " {move || if basic_checked.get() { "ON" } else { "OFF" }}</p>

            <SwitchRoot checked=basic_checked>
                <SwitchThumb />
            </SwitchRoot>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Default On"</h3>
            <p>"State: " {move || if default_on.get() { "ON" } else { "OFF" }}</p>

            <SwitchRoot checked=default_on>
                <SwitchThumb />
            </SwitchRoot>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Disabled States"</h3>

            <div style:display="flex" style:gap="24px" style:align-items="center">
                <div>
                    <p style:margin-bottom="8px">"Disabled (off)"</p>
                    <div style:opacity="0.5">
                        <SwitchRoot checked=disabled_off disabled=true>
                            <SwitchThumb />
                        </SwitchRoot>
                    </div>
                </div>

                <div>
                    <p style:margin-bottom="8px">"Disabled (on)"</p>
                    <div style:opacity="0.5">
                        <SwitchRoot checked=disabled_on disabled=true>
                            <SwitchThumb />
                        </SwitchRoot>
                    </div>
                </div>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Switch with Label"</h3>

            <div style:display="flex" style:align-items="center" style:gap="12px">
                <SwitchRoot checked=labeled_checked>
                    <SwitchThumb />
                </SwitchRoot>
                <label
                    style:cursor="pointer"
                    on:click=move |_| labeled_checked.update(|c| *c = !*c)
                >
                    "Enable notifications"
                </label>
            </div>
            <p style:margin-top="8px" style:color="#666">
                "Notifications are " {move || if labeled_checked.get() { "enabled" } else { "disabled" }}
            </p>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Keyboard Navigation"</h3>
            <ul>
                <li>"Tab: Focus the switch"</li>
                <li>"Space/Enter: Toggle the switch"</li>
            </ul>
        </section>
    }
}
