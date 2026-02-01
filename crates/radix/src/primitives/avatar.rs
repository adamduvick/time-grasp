use leptos::html::{Img, Span};
use leptos::prelude::*;
use leptos::tachys::dom::window;
use wasm_bindgen::JsCast;

/// Status of the image loading process.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ImageLoadingStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}

impl ImageLoadingStatus {
    fn as_str(&self) -> &'static str {
        match self {
            ImageLoadingStatus::Idle => "idle",
            ImageLoadingStatus::Loading => "loading",
            ImageLoadingStatus::Loaded => "loaded",
            ImageLoadingStatus::Error => "error",
        }
    }
}

/// Context shared between Avatar components.
#[derive(Clone, Copy)]
struct AvatarContext {
    status: RwSignal<ImageLoadingStatus>,
}

/// Root container for the avatar. Renders as a span and provides context.
#[component]
pub fn AvatarRoot(
    /// Reference to the root span element.
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// The avatar content (typically AvatarImage and AvatarFallback).
    children: ChildrenFn,
) -> impl IntoView {
    let status = RwSignal::new(ImageLoadingStatus::Idle);
    let ctx = AvatarContext { status };

    provide_context(ctx);

    // Circular avatar with default size, centered content
    let style = "display:inline-flex;align-items:center;justify-content:center;\
                 width:48px;height:48px;border-radius:50%;overflow:hidden;\
                 vertical-align:middle;user-select:none";

    view! {
        <span
            node_ref=node_ref
            style=style
            data-radix-avatar-root=""
        >
            {children()}
        </span>
    }
}

/// Image element that only renders visibly after loading completes.
#[component]
pub fn AvatarImage(
    /// Image source URL.
    #[prop(into)]
    src: Signal<String>,

    /// Alt text for the image.
    #[prop(default = "".into(), into)]
    alt: Signal<String>,

    /// Reference to the img element.
    #[prop(optional)]
    node_ref: NodeRef<Img>,
) -> impl IntoView {
    let ctx = use_context::<AvatarContext>().expect("AvatarImage must be used within AvatarRoot");

    // Set loading status when src changes
    Effect::new(move |_| {
        let _ = src.get();
        ctx.status.set(ImageLoadingStatus::Loading);
    });

    let on_load = move |_| {
        ctx.status.set(ImageLoadingStatus::Loaded);
    };

    let on_error = move |_| {
        ctx.status.set(ImageLoadingStatus::Error);
    };

    // Only show when loaded
    let style = move || {
        let display = if ctx.status.get() == ImageLoadingStatus::Loaded {
            "block"
        } else {
            "none"
        };
        format!(
            "display:{};width:100%;height:100%;object-fit:cover",
            display
        )
    };

    let state_attr = move || ctx.status.get().as_str();

    view! {
        <img
            node_ref=node_ref
            src=src
            alt=alt
            style=style
            data-radix-avatar-image=""
            data-state=state_attr
            on:load=on_load
            on:error=on_error
        />
    }
}

/// Fallback content shown while image is loading or on error.
#[component]
pub fn AvatarFallback(
    /// Delay in milliseconds before showing fallback (prevents flash on fast loads).
    #[prop(optional)]
    delay_ms: Option<u32>,

    /// Reference to the fallback span element.
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// The fallback content (e.g., initials or icon).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx =
        use_context::<AvatarContext>().expect("AvatarFallback must be used within AvatarRoot");

    let can_render = RwSignal::new(delay_ms.is_none());
    let timer_id: StoredValue<Option<i32>> = StoredValue::new(None);

    // Handle delayed rendering
    Effect::new(move |_| {
        let status = ctx.status.get();
        let should_show = status != ImageLoadingStatus::Loaded;

        if let Some(delay) = delay_ms {
            if should_show && !can_render.get() {
                // Start timer to show fallback after delay
                let callback = wasm_bindgen::closure::Closure::once(move || {
                    can_render.set(true);
                });
                let id = window()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap_or(0);
                callback.forget();
                timer_id.set_value(Some(id));
            } else if !should_show {
                // Image loaded, hide fallback and cancel timer
                can_render.set(false);
                if let Some(id) = timer_id.get_value() {
                    window().clear_timeout_with_handle(id);
                    timer_id.set_value(None);
                }
            }
        } else {
            can_render.set(should_show);
        }
    });

    let style = "display:flex;align-items:center;justify-content:center;\
                 width:100%;height:100%;background:#e0e0e0;font-size:16px;font-weight:500";

    let state_attr = move || ctx.status.get().as_str();

    let visible = move || {
        let status = ctx.status.get();
        status != ImageLoadingStatus::Loaded && can_render.get()
    };

    view! {
        <Show when=visible>
            <span
                node_ref=node_ref
                style=style
                data-radix-avatar-fallback=""
                data-state=state_attr
            >
                {children()}
            </span>
        </Show>
    }
}
