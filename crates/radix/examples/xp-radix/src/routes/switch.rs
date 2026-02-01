use leptos::prelude::*;
use radix::{SwitchRoot, SwitchThumb};

#[component]
pub fn SwitchExample() -> impl IntoView {
    // Controlled state for various examples
    let notifications = RwSignal::new(false);
    let dark_mode = RwSignal::new(true);
    let airplane = RwSignal::new(false);
    let wifi = RwSignal::new(true);
    let bluetooth = RwSignal::new(true);

    // Settings panel state
    let push = RwSignal::new(true);
    let email = RwSignal::new(true);
    let sms = RwSignal::new(false);
    let marketing = RwSignal::new(false);

    view! {
        <h1>"Switch"</h1>
        <p>
            "Toggle switch for boolean settings. Different from Toggle button - has "
            "sliding switch visual metaphor and role=\"switch\"."
        </p>

        // Basic Switch
        <div class="example-section">
            <h2>"Basic Switch"</h2>
            <div style="display: flex; align-items: center; gap: 0.75rem">
                <SwitchRoot class="switch-root" checked=notifications>
                    <SwitchThumb class="switch-thumb" />
                </SwitchRoot>
                <label>"Enable notifications"</label>
            </div>
        </div>

        // Default Checked
        <div class="example-section">
            <h2>"Default Checked"</h2>
            <div style="display: flex; align-items: center; gap: 0.75rem">
                <SwitchRoot class="switch-root" checked=dark_mode>
                    <SwitchThumb class="switch-thumb" />
                </SwitchRoot>
                <label>"Dark mode"</label>
            </div>
        </div>

        // Controlled Switches
        <div class="example-section">
            <h2>"Controlled Switches"</h2>
            <div style="display: flex; flex-direction: column; gap: 1rem">
                <div style="display: flex; align-items: center; gap: 0.75rem">
                    <SwitchRoot class="switch-root" checked=airplane>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                    <label>"Airplane Mode"</label>
                </div>

                <div style:display="flex" style:align-items="center" style:gap="0.75rem" style:opacity=move || if airplane.get() { "0.5" } else { "1" }>
                    <SwitchRoot class="switch-root" checked=wifi disabled=airplane>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                    <label>
                        "Wi-Fi"
                        <Show when=move || airplane.get()>
                            " (disabled in airplane mode)"
                        </Show>
                    </label>
                </div>

                <div style:display="flex" style:align-items="center" style:gap="0.75rem" style:opacity=move || if airplane.get() { "0.5" } else { "1" }>
                    <SwitchRoot class="switch-root" checked=bluetooth disabled=airplane>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                    <label>
                        "Bluetooth"
                        <Show when=move || airplane.get()>
                            " (disabled in airplane mode)"
                        </Show>
                    </label>
                </div>
            </div>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "State: Airplane=" {move || airplane.get().to_string()}
                ", Wi-Fi=" {move || wifi.get().to_string()}
                ", Bluetooth=" {move || bluetooth.get().to_string()}
            </p>
        </div>

        // Disabled States
        <div class="example-section">
            <h2>"Disabled States"</h2>
            <div style="display: flex; flex-direction: column; gap: 1rem">
                <div style="display: flex; align-items: center; gap: 0.75rem">
                    <SwitchRoot class="switch-root" checked=RwSignal::new(false) disabled=true>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                    <label style="color: var(--color-text-muted)">"Disabled (off)"</label>
                </div>

                <div style="display: flex; align-items: center; gap: 0.75rem">
                    <SwitchRoot class="switch-root" checked=RwSignal::new(true) disabled=true>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                    <label style="color: var(--color-text-muted)">"Disabled (on)"</label>
                </div>
            </div>
        </div>

        // Settings Panel Example
        <div class="example-section">
            <h2>"Settings Panel Example"</h2>
            <div style="border: 1px solid var(--color-border); border-radius: var(--radius); overflow: hidden">
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem 1rem; border-bottom: 1px solid var(--color-border)">
                    <label>"Push Notifications"</label>
                    <SwitchRoot class="switch-root" checked=push>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                </div>
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem 1rem; border-bottom: 1px solid var(--color-border)">
                    <label>"Email Notifications"</label>
                    <SwitchRoot class="switch-root" checked=email>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                </div>
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem 1rem; border-bottom: 1px solid var(--color-border)">
                    <label>"SMS Notifications"</label>
                    <SwitchRoot class="switch-root" checked=sms>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                </div>
                <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.75rem 1rem">
                    <label>"Marketing Emails"</label>
                    <SwitchRoot class="switch-root" checked=marketing>
                        <SwitchThumb class="switch-thumb" />
                    </SwitchRoot>
                </div>
            </div>
        </div>

        // Switch vs Toggle
        <div class="example-section">
            <h2>"Switch vs Toggle"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Switch"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Toggle"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Visual metaphor"</td>
                        <td style="padding: 0.5rem">"Sliding switch"</td>
                        <td style="padding: 0.5rem">"Pressed button"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"ARIA role"</td>
                        <td style="padding: 0.5rem">"role=\"switch\""</td>
                        <td style="padding: 0.5rem">"aria-pressed"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"data-state"</td>
                        <td style="padding: 0.5rem">"checked/unchecked"</td>
                        <td style="padding: 0.5rem">"on/off"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Use case"</td>
                        <td style="padding: 0.5rem">"Settings, preferences"</td>
                        <td style="padding: 0.5rem">"Toolbar buttons"</td>
                    </tr>
                </tbody>
            </table>
        </div>

        // Data Attributes
        <div class="example-section">
            <h2>"Data Attributes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"data-state"</code>" on Root and Thumb: \"checked\" | \"unchecked\""
                </li>
                <li>
                    <code>"data-disabled"</code>" - Present when disabled"
                </li>
                <li>
                    <code>"role=\"switch\""</code>" - Proper switch semantics"
                </li>
                <li>
                    <code>"aria-checked"</code>" - Accessibility state"
                </li>
            </ul>
        </div>
    }
}
