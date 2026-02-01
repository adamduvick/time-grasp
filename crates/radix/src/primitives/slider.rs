use leptos::html::Div;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

/// Slider orientation
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Context shared between Slider components
#[derive(Clone, Copy)]
struct SliderContext {
    value: RwSignal<f64>,
    min: Signal<f64>,
    max: Signal<f64>,
    step: Signal<f64>,
    orientation: Signal<SliderOrientation>,
    disabled: Signal<bool>,
    track_ref: NodeRef<Div>,
    thumb_ref: RwSignal<Option<NodeRef<Div>>>,
}

impl SliderContext {
    fn provide(self) {
        provide_context(self);
    }

    fn expect() -> Self {
        use_context().expect("SliderRange must be used within SliderRoot")
    }

    /// Clamp and step-align a value
    fn clamp_value(&self, val: f64) -> f64 {
        let min = self.min.get_untracked();
        let max = self.max.get_untracked();
        let step = self.step.get_untracked();

        // Round to nearest step
        let stepped = ((val - min) / step).round() * step + min;
        stepped.clamp(min, max)
    }

    /// Update value with clamping
    fn set_value(&self, val: f64) {
        let clamped = self.clamp_value(val);
        self.value.set(clamped);
    }

    /// Focus the thumb element for keyboard navigation
    fn focus_thumb(&self) {
        if let Some(thumb_ref) = self.thumb_ref.get() {
            if let Some(el) = thumb_ref.get() {
                _ = el.focus();
            }
        }
    }

    fn orientation_attr(&self) -> Memo<&'static str> {
        let orientation = self.orientation;
        Memo::new(move |_| match orientation.get() {
            SliderOrientation::Horizontal => "horizontal",
            SliderOrientation::Vertical => "vertical",
        })
    }

    fn on_pointer_down(self) -> impl Fn(PointerEvent) {
        move |ev: web_sys::PointerEvent| {
            {
                if self.disabled.get() {
                    return;
                }

                ev.prevent_default();

                let Some(track_el) = self.track_ref.get() else {
                    return;
                };

                let rect = track_el.get_bounding_client_rect();
                let orientation = self.orientation.get();

                let percent = match orientation {
                    SliderOrientation::Horizontal => {
                        let x = ev.client_x() as f64 - rect.left();
                        (x / rect.width()).clamp(0.0, 1.0)
                    }
                    SliderOrientation::Vertical => {
                        // For vertical, 0% is at bottom, 100% at top
                        let y = ev.client_y() as f64 - rect.top();
                        (1.0 - y / rect.height()).clamp(0.0, 1.0)
                    }
                };

                let min = self.min.get();
                let max = self.max.get();
                let new_value = min + percent * (max - min);
                self.set_value(new_value);

                // Focus the thumb for keyboard navigation
                self.focus_thumb();
            }
        }
    }
}

/// Root container for the slider. Provides context and manages value state.
#[component]
pub fn SliderRoot(
    /// Controlled value of the slider.
    value: RwSignal<f64>,

    /// Minimum value. Default is 0.
    #[prop(default = 0.0.into(), into)]
    min: Signal<f64>,

    /// Maximum value. Default is 100.
    #[prop(default = 100.0.into(), into)]
    max: Signal<f64>,

    /// Step increment. Default is 1.
    #[prop(default = 1.0.into(), into)]
    step: Signal<f64>,

    /// Slider orientation. Default is `Horizontal`.
    #[prop(default = SliderOrientation::Horizontal.into(), into)]
    orientation: Signal<SliderOrientation>,

    /// Whether the slider is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The slider components (track, thumb).
    children: ChildrenFn,
) -> impl IntoView {
    let track_ref = NodeRef::<Div>::new();
    let thumb_ref = RwSignal::new(None);

    let ctx = SliderContext {
        value,
        min,
        max,
        step,
        orientation,
        disabled,
        track_ref,
        thumb_ref,
    };
    ctx.provide();

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            data-radix-slider-root=""
            data-orientation=ctx.orientation_attr()
            data-disabled=move || disabled.get().then_some("")
        >
            {children()}
        </div>
    }
}

/// Background track rail for the slider.
#[component]
pub fn SliderTrack(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the track element.
    #[prop(optional, into)]
    style: Option<String>,

    /// The track content (typically SliderRange).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = SliderContext::expect();

    view! {
        <div
            node_ref=ctx.track_ref
            class=class
            style=style
            data-radix-slider-track=""
            data-orientation=ctx.orientation_attr()
            data-disabled=move || ctx.disabled.get().then_some("")
            on:pointerdown=ctx.on_pointer_down()
        >
            {children()}
        </div>
    }
}

/// Filled portion of the track showing the current value.
#[component]
pub fn SliderRange(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the range element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the range element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx = SliderContext::expect();

    // Calculate percentage filled
    let percent = Signal::derive(move || {
        let value = ctx.value.get();
        let min = ctx.min.get();
        let max = ctx.max.get();

        if max <= min {
            return 0.0;
        }

        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    });

    // Dynamic width/height based on orientation and value, merged with user style
    let computed_style = move || {
        let size_style = match ctx.orientation.get() {
            SliderOrientation::Horizontal => format!("width: {}%", percent.get()),
            SliderOrientation::Vertical => format!("height: {}%", percent.get()),
        };
        match &style {
            Some(s) => format!("{}; {}", size_style, s),
            None => size_style,
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=computed_style
            data-radix-slider-range=""
            data-orientation=ctx.orientation_attr()
            data-disabled=move || ctx.disabled.get().then_some("")
        />
    }
}

/// Draggable thumb handle for the slider.
#[component]
pub fn SliderThumb(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the thumb element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the thumb element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx = SliderContext::expect();

    // Register thumb ref with context for focus management
    ctx.thumb_ref.set(Some(node_ref));

    // Track drag state
    let is_dragging = RwSignal::new(false);

    // Calculate thumb position as percentage
    let percent = Signal::derive(move || {
        let value = ctx.value.get();
        let min = ctx.min.get();
        let max = ctx.max.get();

        if max <= min {
            return 0.0;
        }

        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    });

    // Pointer event handlers for drag
    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        if ctx.disabled.get() {
            return;
        }

        ev.prevent_default();
        ev.stop_propagation();

        if let Some(target) = ev.target()
            && let Ok(el) = target.dyn_into::<web_sys::Element>()
        {
            _ = el.set_pointer_capture(ev.pointer_id());
        }

        is_dragging.set(true);

        // Focus the thumb for keyboard navigation
        if let Some(el) = node_ref.get() {
            _ = el.focus();
        }
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        if !is_dragging.get() || ctx.disabled.get() {
            return;
        }

        ev.prevent_default();

        let Some(track_el) = ctx.track_ref.get() else {
            return;
        };

        let rect = track_el.get_bounding_client_rect();
        let orientation = ctx.orientation.get();

        let percent = match orientation {
            SliderOrientation::Horizontal => {
                let x = ev.client_x() as f64 - rect.left();
                (x / rect.width()).clamp(0.0, 1.0)
            }
            SliderOrientation::Vertical => {
                let y = ev.client_y() as f64 - rect.top();
                (1.0 - y / rect.height()).clamp(0.0, 1.0)
            }
        };

        let min = ctx.min.get();
        let max = ctx.max.get();
        let new_value = min + percent * (max - min);
        ctx.set_value(new_value);
    };

    let on_pointer_up = move |ev: web_sys::PointerEvent| {
        if let Some(target) = ev.target()
            && let Ok(el) = target.dyn_into::<web_sys::Element>()
        {
            _ = el.release_pointer_capture(ev.pointer_id());
        }

        is_dragging.set(false);
    };

    // Keyboard event handler
    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if ctx.disabled.get() {
            return;
        }

        let step = ctx.step.get();
        let min = ctx.min.get();
        let max = ctx.max.get();
        let current = ctx.value.get();

        let large_step = step * 10.0;

        let new_value = match ev.key().as_str() {
            "ArrowRight" | "ArrowUp" => Some(current + step),
            "ArrowLeft" | "ArrowDown" => Some(current - step),
            "PageUp" => Some(current + large_step),
            "PageDown" => Some(current - large_step),
            "Home" => Some(min),
            "End" => Some(max),
            _ => None,
        };

        if let Some(val) = new_value {
            ev.prevent_default();
            ctx.set_value(val);
        }
    };

    let state_attr = move || {
        if is_dragging.get() {
            "dragging"
        } else {
            "idle"
        }
    };

    // Position the thumb based on value percentage, merged with user style
    let computed_style = move || {
        let pos_style = match ctx.orientation.get() {
            SliderOrientation::Horizontal => format!("left: {}%", percent.get()),
            SliderOrientation::Vertical => format!("bottom: {}%", percent.get()),
        };
        match &style {
            Some(s) => format!("{}; {}", pos_style, s),
            None => pos_style,
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            tabindex=move || if ctx.disabled.get() { -1 } else { 0 }
            role="slider"
            aria-valuemin=move || ctx.min.get()
            aria-valuemax=move || ctx.max.get()
            aria-valuenow=move || ctx.value.get()
            aria-orientation=ctx.orientation_attr()
            aria-disabled=move || ctx.disabled.get().then_some("true")
            style=computed_style
            data-radix-slider-thumb=""
            data-orientation=ctx.orientation_attr()
            data-state=state_attr
            data-disabled=move || ctx.disabled.get().then_some("")
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:keydown=on_key_down
        />
    }
}
