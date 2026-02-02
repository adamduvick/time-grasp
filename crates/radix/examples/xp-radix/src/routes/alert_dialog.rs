use leptos::prelude::*;
use leptos::web_sys;
use radix::{
    AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogOverlay, AlertDialogPortal, AlertDialogRoot, AlertDialogTitle, AlertDialogTrigger,
};

/// AlertDialog Primitive
///
/// RADIX PROVIDES:
/// - Same features as Dialog (portal, focus trap, scroll lock)
/// - Requires explicit action to close (no click-outside dismiss)
/// - Cancel and Action button parts for semantic clarity
/// - role="alertdialog" for screen readers
/// - Esc key closes by default
/// - Initial focus goes to Cancel button
///
/// USER MUST IMPLEMENT:
/// - All visual styling (same as Dialog)
/// - Confirmation/cancel button styling
/// - Destructive action styling if needed
///
/// USE WHEN: Action requires confirmation and shouldn't be dismissed accidentally

#[component]
pub fn AlertDialogExample() -> impl IntoView {
    view! {
        <h1>"AlertDialog"</h1>
        <p>
            "Modal for confirmations. Unlike Dialog, it requires explicit action to "
            "close (no click-outside dismiss). Used for destructive or irreversible actions."
        </p>

        <div class="example-section">
            <h2>"Basic Confirmation"</h2>
            <AlertDialogRoot>
                <AlertDialogTrigger class="trigger-button">
                    "Delete Account"
                </AlertDialogTrigger>
                <AlertDialogPortal>
                    <AlertDialogOverlay class="dialog-overlay" />
                    <AlertDialogContent class="dialog-content">
                        <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "This action cannot be undone. This will permanently delete your "
                            "account and remove all your data from our servers."
                        </AlertDialogDescription>
                        <div style="display: flex; gap: 0.5rem; margin-top: 1rem">
                            <AlertDialogCancel class="trigger-button">
                                "Cancel"
                            </AlertDialogCancel>
                            <AlertDialogAction
                                class="trigger-button"
                                style="background: #dc2626"
                                on:click=move |_| {
                                    let _ = web_sys::window()
                                        .unwrap()
                                        .alert_with_message("Account deleted!");
                                }
                            >
                                "Yes, delete account"
                            </AlertDialogAction>
                        </div>
                    </AlertDialogContent>
                </AlertDialogPortal>
            </AlertDialogRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try clicking outside the dialog - it won't close (unlike regular Dialog)"
            </p>
        </div>

        <div class="example-section">
            <h2>"Save Changes Prompt"</h2>
            <AlertDialogRoot>
                <AlertDialogTrigger class="trigger-button">
                    "Close Editor"
                </AlertDialogTrigger>
                <AlertDialogPortal>
                    <AlertDialogOverlay class="dialog-overlay" />
                    <AlertDialogContent class="dialog-content">
                        <AlertDialogTitle>"Unsaved Changes"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "You have unsaved changes. Do you want to save before closing?"
                        </AlertDialogDescription>
                        <div style="display: flex; gap: 0.5rem; margin-top: 1rem; justify-content: flex-end">
                            <AlertDialogCancel class="trigger-button">
                                "Cancel"
                            </AlertDialogCancel>
                            <AlertDialogAction
                                class="trigger-button"
                                style="background: var(--color-border)"
                                on:click=move |_| {
                                    let _ = web_sys::window()
                                        .unwrap()
                                        .alert_with_message("Discarded!");
                                }
                            >
                                "Don't Save"
                            </AlertDialogAction>
                            <AlertDialogAction
                                class="trigger-button"
                                on:click=move |_| {
                                    let _ = web_sys::window()
                                        .unwrap()
                                        .alert_with_message("Saved!");
                                }
                            >
                                "Save"
                            </AlertDialogAction>
                        </div>
                    </AlertDialogContent>
                </AlertDialogPortal>
            </AlertDialogRoot>
        </div>

        <div class="example-section">
            <h2>"Dialog vs AlertDialog"</h2>
            <table style="width: 100%; font-size: 0.875rem; border-collapse: collapse">
                <thead>
                    <tr>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Feature"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"Dialog"</th>
                        <th style="text-align: left; padding: 0.5rem; border-bottom: 1px solid var(--color-border)">"AlertDialog"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="padding: 0.5rem">"Click outside closes"</td>
                        <td style="padding: 0.5rem">"Yes"</td>
                        <td style="padding: 0.5rem">"No"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"ARIA role"</td>
                        <td style="padding: 0.5rem">"dialog"</td>
                        <td style="padding: 0.5rem">"alertdialog"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Use case"</td>
                        <td style="padding: 0.5rem">"General modals"</td>
                        <td style="padding: 0.5rem">"Confirmations"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Cancel/Action parts"</td>
                        <td style="padding: 0.5rem">"No (just Close)"</td>
                        <td style="padding: 0.5rem">"Yes"</td>
                    </tr>
                    <tr>
                        <td style="padding: 0.5rem">"Initial focus"</td>
                        <td style="padding: 0.5rem">"First focusable element"</td>
                        <td style="padding: 0.5rem">"Cancel button"</td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="example-section">
            <h2>"Keyboard Navigation"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li><code>"Escape"</code>" - Close the alert dialog"</li>
                <li><code>"Tab"</code>" - Cycle focus within dialog (trapped)"</li>
                <li><code>"Shift+Tab"</code>" - Cycle focus backwards"</li>
                <li>"Click outside - Does NOT close (requires explicit action)"</li>
            </ul>
        </div>
    }
}
