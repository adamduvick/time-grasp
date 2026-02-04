use leptos::prelude::*;
use radix::{
    ToastAction, ToastClose, ToastDescription, ToastProvider, ToastRoot, ToastTitle, ToastViewport,
};

/// Toast Primitive
///
/// RADIX PROVIDES:
/// - Provider/Viewport pattern for toast positioning
/// - Auto-dismiss with configurable duration
/// - Pause on hover/focus
/// - Swipe to dismiss (touch devices)
/// - Multiple toasts with stacking
/// - data-state="open" | "closed"
/// - data-swipe="start" | "move" | "cancel" | "end"
/// - Proper ARIA: role="status", aria-live
/// - Action and Close parts
///
/// USER MUST IMPLEMENT:
/// - Toast styling
/// - Viewport positioning
/// - Entry/exit animations (using data-state)
/// - Swipe animations (using data-swipe)

#[component]
pub fn ToastExample() -> impl IntoView {
    // State for basic toast
    let open1 = RwSignal::new(false);
    // State for toast with action
    let open2 = RwSignal::new(false);
    // State for long duration toast
    let open3 = RwSignal::new(false);

    view! {
        <ToastProvider swipe_direction=Signal::derive(|| radix::ToastSwipeDirection::Right)>
            <h1>"Toast"</h1>
            <p>
                "Temporary notifications with auto-dismiss. Radix handles timing, "
                "stacking, swipe gestures, and ARIA. User styles appearance and animations."
            </p>

            <div class="example-section">
                <h2>"Basic Toast"</h2>
                <button
                    class="trigger-button"
                    on:click=move |_| {
                        open1.set(false);
                        // Small delay to allow close animation
                        set_timeout(move || open1.set(true), std::time::Duration::from_millis(100));
                    }
                >
                    "Show Toast"
                </button>

                <ToastRoot class="toast-root" open=open1 on_open_change=Callback::new(move |v| open1.set(v))>
                    <ToastTitle class="toast-title">"Notification"</ToastTitle>
                    <ToastDescription class="toast-description">
                        "This is a basic toast notification."
                    </ToastDescription>
                    <ToastClose class="toast-close">"×"</ToastClose>
                </ToastRoot>
            </div>

            <div class="example-section">
                <h2>"Toast with Action"</h2>
                <button
                    class="trigger-button"
                    on:click=move |_| {
                        open2.set(false);
                        set_timeout(move || open2.set(true), std::time::Duration::from_millis(100));
                    }
                >
                    "Show Toast with Action"
                </button>

                <ToastRoot class="toast-root" open=open2 on_open_change=Callback::new(move |v| open2.set(v))>
                    <ToastTitle class="toast-title">"File deleted"</ToastTitle>
                    <ToastDescription class="toast-description">
                        "\"document.pdf\" has been moved to trash."
                    </ToastDescription>
                    <ToastAction
                        class="toast-action"
                        alt_text="Undo delete action"
                    >
                        "Undo"
                    </ToastAction>
                    <ToastClose class="toast-close">"×"</ToastClose>
                </ToastRoot>
            </div>

            <div class="example-section">
                <h2>"Custom Duration"</h2>
                <button
                    class="trigger-button"
                    on:click=move |_| {
                        open3.set(false);
                        set_timeout(move || open3.set(true), std::time::Duration::from_millis(100));
                    }
                >
                    "Show Toast (10s duration)"
                </button>

                <ToastRoot
                    class="toast-root"
                    open=open3
                    on_open_change=Callback::new(move |v| open3.set(v))
                    duration=Signal::derive(|| 10000u32)
                >
                    <ToastTitle class="toast-title">"Long Toast"</ToastTitle>
                    <ToastDescription class="toast-description">
                        "This toast will stay for 10 seconds. Hover to pause the timer."
                    </ToastDescription>
                    <ToastClose class="toast-close">"×"</ToastClose>
                </ToastRoot>
            </div>

            <div class="example-section">
                <h2>"Multiple Toasts"</h2>
                <MultiToastDemo />
            </div>

            <div class="example-section">
                <h2>"Toast Types"</h2>
                <ToastTypesDemo />
            </div>

            <div class="example-section">
                <h2>"Configuration"</h2>
                <ul style="font-size: 0.875rem; line-height: 1.8">
                    <li>
                        <code>"duration"</code>" - Auto-dismiss time in ms (default: 5000)"
                    </li>
                    <li>
                        <code>"swipe_direction"</code>" - Right | Left | Up | Down"
                    </li>
                    <li>
                        <code>"swipe_threshold"</code>" - Distance in px to trigger dismiss"
                    </li>
                    <li>
                        "Timer pauses on hover/focus, resumes on leave"
                    </li>
                </ul>
            </div>

            <div class="example-section">
                <h2>"Data Attributes"</h2>
                <ul style="font-size: 0.875rem; line-height: 1.8">
                    <li>
                        <code>"data-state"</code>" - \"open\" | \"closed\" (for animations)"
                    </li>
                    <li>
                        <code>"data-swipe"</code>" - \"start\" | \"move\" | \"cancel\" | \"end\""
                    </li>
                    <li>
                        <code>"--radix-toast-swipe-move-x/y"</code>" - CSS variable for swipe position"
                    </li>
                </ul>
            </div>

            <ToastViewport class="toast-viewport" />
        </ToastProvider>
    }
}

#[component]
fn MultiToastDemo() -> impl IntoView {
    let toasts = RwSignal::new(Vec::<u32>::new());
    let counter = RwSignal::new(0u32);

    let add_toast = move |_| {
        counter.update(|c| *c += 1);
        let id = counter.get();
        toasts.update(|t| t.push(id));
    };

    let remove_toast = move |id: u32| {
        toasts.update(|t| t.retain(|&x| x != id));
    };

    view! {
        <button class="trigger-button" on:click=add_toast>
            "Add Toast"
        </button>
        <span style="margin-left: 1rem; font-size: 0.875rem">
            "Active toasts: "
            {move || toasts.get().len()}
        </span>

        <For
            each=move || toasts.get()
            key=|id| *id
            children=move |id| {
                view! {
                    <ToastRoot
                        class="toast-root"
                        on_open_change=Callback::new(move |open: bool| {
                            if !open {
                                remove_toast(id);
                            }
                        })
                    >
                        <ToastTitle class="toast-title">
                            {format!("Toast #{}", id)}
                        </ToastTitle>
                        <ToastDescription class="toast-description">
                            "Multiple toasts stack in the viewport."
                        </ToastDescription>
                        <ToastClose class="toast-close">"×"</ToastClose>
                    </ToastRoot>
                }
            }
        />
    }
}

#[component]
fn ToastTypesDemo() -> impl IntoView {
    let success_open = RwSignal::new(false);
    let error_open = RwSignal::new(false);
    let warning_open = RwSignal::new(false);

    view! {
        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap">
            <button
                class="trigger-button"
                style="background: #22c55e"
                on:click=move |_| success_open.set(true)
            >
                "Success"
            </button>
            <button
                class="trigger-button"
                style="background: #ef4444"
                on:click=move |_| error_open.set(true)
            >
                "Error"
            </button>
            <button
                class="trigger-button"
                style="background: #f59e0b"
                on:click=move |_| warning_open.set(true)
            >
                "Warning"
            </button>

            <ToastRoot
                class="toast-root toast-success"
                open=success_open
                on_open_change=Callback::new(move |v| success_open.set(v))
            >
                <ToastTitle class="toast-title">"Success!"</ToastTitle>
                <ToastDescription class="toast-description">
                    "Your changes have been saved."
                </ToastDescription>
                <ToastClose class="toast-close">"×"</ToastClose>
            </ToastRoot>

            <ToastRoot
                class="toast-root toast-error"
                open=error_open
                on_open_change=Callback::new(move |v| error_open.set(v))
            >
                <ToastTitle class="toast-title">"Error"</ToastTitle>
                <ToastDescription class="toast-description">
                    "Something went wrong. Please try again."
                </ToastDescription>
                <ToastClose class="toast-close">"×"</ToastClose>
            </ToastRoot>

            <ToastRoot
                class="toast-root toast-warning"
                open=warning_open
                on_open_change=Callback::new(move |v| warning_open.set(v))
            >
                <ToastTitle class="toast-title">"Warning"</ToastTitle>
                <ToastDescription class="toast-description">
                    "Your session will expire in 5 minutes."
                </ToastDescription>
                <ToastClose class="toast-close">"×"</ToastClose>
            </ToastRoot>
        </div>
    }
}
