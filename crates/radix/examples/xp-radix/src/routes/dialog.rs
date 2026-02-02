use leptos::prelude::*;
use leptos::web_sys;
use radix::{
    DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogRoot,
    DialogTitle, DialogTrigger,
};

/// Dialog Primitive
///
/// RADIX PROVIDES:
/// - Portal rendering (content renders outside DOM hierarchy)
/// - Focus trap (Tab cycles within dialog)
/// - Focus restoration (returns focus to trigger on close)
/// - Esc key to close
/// - aria-labelledby, aria-describedby associations
/// - data-state="open" | "closed" on all parts
///
/// USER MUST IMPLEMENT:
/// - Overlay styling (position, background)
/// - Content positioning (centered, slide-in, etc.)
/// - Visual appearance of all elements
/// - Animations (can use data-state for enter/exit)

#[component]
pub fn DialogExample() -> impl IntoView {
    view! {
        <h1>"Dialog"</h1>
        <p>
            "Modal overlay pattern. Radix handles focus trap, portal, "
            "and keyboard interactions. User styles everything visual."
        </p>

        <div class="example-section">
            <h2>"Basic Dialog"</h2>
            <DialogRoot>
                <DialogTrigger class="trigger-button">
                    "Open Dialog"
                </DialogTrigger>
                <DialogPortal>
                    <DialogOverlay class="dialog-overlay" />
                    <DialogContent class="dialog-content">
                        <DialogTitle>"Dialog Title"</DialogTitle>
                        <DialogDescription>
                            "This is a description of the dialog content. Radix automatically "
                            "associates this with aria-describedby."
                        </DialogDescription>
                        <p>
                            "Try pressing Tab to see focus trapped within. Press Esc or click "
                            "outside to close."
                        </p>
                        <DialogClose class="trigger-button">
                            "Close"
                        </DialogClose>
                        <DialogClose class="dialog-close">
                            "×"
                        </DialogClose>
                    </DialogContent>
                </DialogPortal>
            </DialogRoot>
        </div>

        <div class="example-section">
            <h2>"Dialog with Form"</h2>
            <DialogRoot>
                <DialogTrigger class="trigger-button">
                    "Edit Profile"
                </DialogTrigger>
                <DialogPortal>
                    <DialogOverlay class="dialog-overlay" />
                    <DialogContent class="dialog-content">
                        <DialogTitle>"Edit Profile"</DialogTitle>
                        <DialogDescription>
                            "Make changes to your profile here."
                        </DialogDescription>
                        <form on:submit=move |ev| {
                            ev.prevent_default();
                            web_sys::window()
                                .unwrap()
                                .alert_with_message("Form submitted!")
                                .unwrap();
                        }>
                            <div style="margin-bottom: 1rem">
                                <label for="name" style="display: block">"Name"</label>
                                <input
                                    id="name"
                                    type="text"
                                    value="John Doe"
                                    style="width: 100%; padding: 0.5rem"
                                />
                            </div>
                            <div style="margin-bottom: 1rem">
                                <label for="email" style="display: block">"Email"</label>
                                <input
                                    id="email"
                                    type="email"
                                    value="john@example.com"
                                    style="width: 100%; padding: 0.5rem"
                                />
                            </div>
                            <div style="display: flex; gap: 0.5rem">
                                <button type="submit" class="trigger-button">
                                    "Save"
                                </button>
                                <DialogClose class="trigger-button">
                                    "Cancel"
                                </DialogClose>
                            </div>
                        </form>
                        <DialogClose class="dialog-close">
                            "×"
                        </DialogClose>
                    </DialogContent>
                </DialogPortal>
            </DialogRoot>
        </div>

        <div class="example-section">
            <h2>"Nested Dialogs"</h2>
            <DialogRoot>
                <DialogTrigger class="trigger-button">
                    "Open Outer Dialog"
                </DialogTrigger>
                <DialogPortal>
                    <DialogOverlay class="dialog-overlay" />
                    <DialogContent class="dialog-content">
                        <DialogTitle>"Outer Dialog"</DialogTitle>
                        <DialogDescription>
                            "This dialog contains another dialog."
                        </DialogDescription>
                        <DialogRoot>
                            <DialogTrigger class="trigger-button">
                                "Open Inner Dialog"
                            </DialogTrigger>
                            <DialogPortal>
                                <DialogOverlay class="dialog-overlay" />
                                <DialogContent class="dialog-content">
                                    <DialogTitle>"Inner Dialog"</DialogTitle>
                                    <DialogDescription>
                                        "This is a nested dialog. Focus is trapped here now."
                                    </DialogDescription>
                                    <DialogClose class="trigger-button">
                                        "Close Inner"
                                    </DialogClose>
                                    <DialogClose class="dialog-close">
                                        "×"
                                    </DialogClose>
                                </DialogContent>
                            </DialogPortal>
                        </DialogRoot>
                        <DialogClose class="dialog-close">
                            "×"
                        </DialogClose>
                    </DialogContent>
                </DialogPortal>
            </DialogRoot>
        </div>

        <div class="example-section">
            <h2>"Controlled Dialog"</h2>
            <ControlledDialogExample />
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"Escape"</code>" - Close the dialog"</li>
                <li><code>"Tab"</code>" - Cycle focus within dialog (trapped)"</li>
                <li><code>"Shift+Tab"</code>" - Cycle focus backwards"</li>
                <li>"Click overlay - Close the dialog"</li>
            </ul>
        </div>
    }
}

#[component]
fn ControlledDialogExample() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div style="display: flex; gap: 1rem; align-items: center">
            <button
                class="trigger-button"
                on:click=move |_| open.set(true)
            >
                "Open from external button"
            </button>
            <span>"Dialog is: " {move || if open.get() { "open" } else { "closed" }}</span>
        </div>
        <DialogRoot open=open>
            <DialogPortal>
                <DialogOverlay class="dialog-overlay" />
                <DialogContent class="dialog-content">
                    <DialogTitle>"Controlled Dialog"</DialogTitle>
                    <DialogDescription>
                        "This dialog's state is controlled externally."
                    </DialogDescription>
                    <p>"The open state is managed by a signal outside the dialog."</p>
                    <DialogClose class="trigger-button">
                        "Close"
                    </DialogClose>
                    <DialogClose class="dialog-close">
                        "×"
                    </DialogClose>
                </DialogContent>
            </DialogPortal>
        </DialogRoot>
    }
}
