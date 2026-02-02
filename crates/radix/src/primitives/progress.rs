use leptos::html::Div;
use leptos::prelude::*;

/// Context shared between Progress components
#[derive(Clone, Copy)]
struct ProgressContext {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

impl ProgressContext {
    fn state(&self) -> Memo<&'static str> {
        let value = self.value;
        let max = self.max;
        Memo::new(move |_| compute_state(value.get(), max.get()))
    }
}

/// Computes the progress state based on value and max
fn compute_state(value: Option<f64>, max: f64) -> &'static str {
    match value {
        None => "indeterminate",
        Some(v) if v >= max => "complete",
        Some(_) => "loading",
    }
}

/// Root container for the progress bar. Acts as the track/background.
///
/// Based on [Radix UI Progress](https://www.radix-ui.com/primitives/docs/components/progress).
#[component]
pub fn ProgressRoot(
    /// Current progress value. None means indeterminate state.
    #[prop(optional, into)]
    value: Option<Signal<Option<f64>>>,

    /// Maximum value (default 100).
    #[prop(default = 100.0.into(), into)]
    max: Signal<f64>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the root element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The progress content (typically ProgressIndicator).
    children: ChildrenFn,
) -> impl IntoView {
    // If value prop not provided, default to indeterminate
    let value_signal = value.unwrap_or_else(|| Signal::derive(|| None));

    let ctx = ProgressContext {
        value: value_signal,
        max,
    };

    provide_context(ctx);

    let state = ctx.state();

    // aria-valuenow should only be set when determinate
    let aria_valuenow = move || value_signal.get().map(|v| v.to_string());

    // data-value should only be set when determinate
    let data_value = move || value_signal.get().map(|v| v.to_string());

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            role="progressbar"
            aria-valuenow=aria_valuenow
            aria-valuemin="0"
            aria-valuemax=move || max.get().to_string()
            data-state=state
            data-value=data_value
            data-max=move || max.get().to_string()
            data-radix-progress-root=""
        >
            {children()}
        </div>
    }
}

/// Visual indicator that shows the filled portion of the progress bar.
#[component]
pub fn ProgressIndicator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the indicator element.
    /// Can be a static string or a reactive signal for dynamic transforms.
    #[prop(optional, into)]
    style: Option<Signal<String>>,

    /// Reference to the indicator element.
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let ctx =
        use_context::<ProgressContext>().expect("ProgressIndicator must be used within ProgressRoot");

    let state = ctx.state();

    // data-value should only be set when determinate
    let data_value = move || ctx.value.get().map(|v| v.to_string());

    let style_attr = move || style.as_ref().map(|s| s.get());

    view! {
        <div
            node_ref=node_ref
            class=class
            style=style_attr
            data-state=state
            data-value=data_value
            data-max=move || ctx.max.get().to_string()
            data-radix-progress-indicator=""
        />
    }
}
