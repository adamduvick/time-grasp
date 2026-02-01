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
    values: RwSignal<Vec<f64>>,
    min: Signal<f64>,
    max: Signal<f64>,
    step: Signal<f64>,
    min_steps_between_thumbs: Signal<u32>,
    orientation: Signal<SliderOrientation>,
    disabled: Signal<bool>,
    track_ref: NodeRef<Div>,
    thumb_count: RwSignal<usize>,
    active_thumb: RwSignal<Option<usize>>,
}

impl SliderContext {
    fn provide(self) {
        provide_context(self);
    }

    fn expect() -> Self {
        use_context().expect("Slider components must be used within SliderRoot")
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

    /// Update a specific thumb's value with clamping and collision prevention
    fn set_thumb_value(&self, index: usize, val: f64) {
        let clamped = self.clamp_value(val);
        let step = self.step.get_untracked();
        let min_gap = step * self.min_steps_between_thumbs.get_untracked() as f64;

        self.values.update(|values| {
            if index >= values.len() {
                return;
            }

            let mut new_val = clamped;

            // Prevent collision with previous thumb
            if index > 0 {
                let prev_val = values[index - 1];
                if new_val < prev_val + min_gap {
                    new_val = prev_val + min_gap;
                }
            }

            // Prevent collision with next thumb
            if index + 1 < values.len() {
                let next_val = values[index + 1];
                if new_val > next_val - min_gap {
                    new_val = next_val - min_gap;
                }
            }

            values[index] = self.clamp_value(new_val);
        });
    }

    /// Find the closest thumb to a given value
    fn find_closest_thumb(&self, target_value: f64) -> usize {
        let values = self.values.get_untracked();
        if values.is_empty() {
            return 0;
        }

        let mut closest_index = 0;
        let mut closest_distance = f64::MAX;

        for (i, &val) in values.iter().enumerate() {
            let distance = (val - target_value).abs();
            if distance < closest_distance {
                closest_distance = distance;
                closest_index = i;
            }
        }

        closest_index
    }

    fn orientation_attr(&self) -> Memo<&'static str> {
        let orientation = self.orientation;
        Memo::new(move |_| match orientation.get() {
            SliderOrientation::Horizontal => "horizontal",
            SliderOrientation::Vertical => "vertical",
        })
    }

    fn percent_from_pointer(&self, ev: &PointerEvent) -> f64 {
        let Some(track_el) = self.track_ref.get() else {
            return 0.0;
        };

        let rect = track_el.get_bounding_client_rect();
        let orientation = self.orientation.get();

        match orientation {
            SliderOrientation::Horizontal => {
                let x = ev.client_x() as f64 - rect.left();
                (x / rect.width()).clamp(0.0, 1.0)
            }
            SliderOrientation::Vertical => {
                let y = ev.client_y() as f64 - rect.top();
                (1.0 - y / rect.height()).clamp(0.0, 1.0)
            }
        }
    }

    fn value_from_percent(&self, percent: f64) -> f64 {
        let min = self.min.get();
        let max = self.max.get();
        min + percent * (max - min)
    }

    fn on_track_pointer_down(self) -> impl Fn(PointerEvent) {
        move |ev: PointerEvent| {
            if self.disabled.get() {
                return;
            }

            ev.prevent_default();

            let percent = self.percent_from_pointer(&ev);
            let target_value = self.value_from_percent(percent);
            let closest_thumb = self.find_closest_thumb(target_value);

            self.set_thumb_value(closest_thumb, target_value);
            self.active_thumb.set(Some(closest_thumb));
        }
    }
}

/// Root container for the slider. Provides context and manages value state.
#[component]
pub fn SliderRoot(
    /// Controlled values of the slider (one per thumb).
    values: RwSignal<Vec<f64>>,

    /// Minimum value. Default is 0.
    #[prop(default = 0.0.into(), into)]
    min: Signal<f64>,

    /// Maximum value. Default is 100.
    #[prop(default = 100.0.into(), into)]
    max: Signal<f64>,

    /// Step increment. Default is 1.
    #[prop(default = 1.0.into(), into)]
    step: Signal<f64>,

    /// Minimum number of steps between thumbs. Default is 0.
    #[prop(default = 0.into(), into)]
    min_steps_between_thumbs: Signal<u32>,

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
    let thumb_count = RwSignal::new(0usize);
    let active_thumb = RwSignal::new(None);

    let ctx = SliderContext {
        values,
        min,
        max,
        step,
        min_steps_between_thumbs,
        orientation,
        disabled,
        track_ref,
        thumb_count,
        active_thumb,
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
            on:pointerdown=ctx.on_track_pointer_down()
        >
            {children()}
        </div>
    }
}

/// Filled portion of the track showing the range between min value and max value (or between thumbs for range sliders).
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

    // Calculate range position and size
    let range_style = Signal::derive(move || {
        let values = ctx.values.get();
        let min = ctx.min.get();
        let max = ctx.max.get();

        if max <= min || values.is_empty() {
            return (0.0, 0.0);
        }

        let range = max - min;

        if values.len() == 1 {
            // Single thumb: range from 0 to value
            let end_percent = ((values[0] - min) / range * 100.0).clamp(0.0, 100.0);
            (0.0, end_percent)
        } else {
            // Multiple thumbs: range between first and last thumb
            let start_percent = ((values[0] - min) / range * 100.0).clamp(0.0, 100.0);
            let end_percent = ((values[values.len() - 1] - min) / range * 100.0).clamp(0.0, 100.0);
            (start_percent, end_percent - start_percent)
        }
    });

    // Dynamic positioning based on orientation
    let computed_style = move || {
        let (start, size) = range_style.get();
        let pos_style = match ctx.orientation.get() {
            SliderOrientation::Horizontal => format!("left: {}%; width: {}%", start, size),
            SliderOrientation::Vertical => format!("bottom: {}%; height: {}%", start, size),
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

    // Get this thumb's index by incrementing the counter
    let thumb_index = ctx.thumb_count.get_untracked();
    ctx.thumb_count.update(|c| *c += 1);

    // Track drag state
    let is_dragging = RwSignal::new(false);

    // Calculate thumb position as percentage
    let percent = Signal::derive(move || {
        let values = ctx.values.get();
        let min = ctx.min.get();
        let max = ctx.max.get();

        if max <= min || thumb_index >= values.len() {
            return 0.0;
        }

        let value = values[thumb_index];
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    });

    // Get current value for ARIA
    let current_value = Signal::derive(move || {
        let values = ctx.values.get();
        if thumb_index < values.len() {
            values[thumb_index]
        } else {
            0.0
        }
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
        ctx.active_thumb.set(Some(thumb_index));

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

        let percent = ctx.percent_from_pointer(&ev);
        let new_value = ctx.value_from_percent(percent);
        ctx.set_thumb_value(thumb_index, new_value);
    };

    let on_pointer_up = move |ev: web_sys::PointerEvent| {
        if let Some(target) = ev.target()
            && let Ok(el) = target.dyn_into::<web_sys::Element>()
        {
            _ = el.release_pointer_capture(ev.pointer_id());
        }

        is_dragging.set(false);
        ctx.active_thumb.set(None);
    };

    // Keyboard event handler
    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if ctx.disabled.get() {
            return;
        }

        let step = ctx.step.get();
        let min = ctx.min.get();
        let max = ctx.max.get();
        let current = current_value.get();

        // Shift+Arrow uses 10x step (matching Radix React behavior)
        let large_step = step * 10.0;
        let effective_step = if ev.shift_key() { large_step } else { step };

        let new_value = match ev.key().as_str() {
            "ArrowRight" | "ArrowUp" => Some(current + effective_step),
            "ArrowLeft" | "ArrowDown" => Some(current - effective_step),
            "PageUp" => Some(current + large_step),
            "PageDown" => Some(current - large_step),
            "Home" => Some(min),
            "End" => Some(max),
            _ => None,
        };

        if let Some(val) = new_value {
            ev.prevent_default();
            ctx.set_thumb_value(thumb_index, val);
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
            aria-valuenow=current_value
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
