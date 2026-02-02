use leptos::html::Button;
use leptos::prelude::*;

/// Tri-state checkbox value: checked, unchecked, or indeterminate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckedState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckedState {
    fn data_state(&self) -> &'static str {
        match self {
            CheckedState::Unchecked => "unchecked",
            CheckedState::Checked => "checked",
            CheckedState::Indeterminate => "indeterminate",
        }
    }

    fn aria_checked(&self) -> &'static str {
        match self {
            CheckedState::Unchecked => "false",
            CheckedState::Checked => "true",
            CheckedState::Indeterminate => "mixed",
        }
    }

    fn is_present(&self) -> bool {
        matches!(self, CheckedState::Checked | CheckedState::Indeterminate)
    }
}

impl From<bool> for CheckedState {
    fn from(checked: bool) -> Self {
        if checked {
            CheckedState::Checked
        } else {
            CheckedState::Unchecked
        }
    }
}

/// Context shared between Checkbox components.
#[derive(Clone, Copy)]
struct CheckboxContext {
    checked: RwSignal<CheckedState>,
    disabled: Signal<bool>,
}

/// Root checkbox button that manages checked state.
#[component]
pub fn CheckboxRoot(
    /// Controlled checked state.
    checked: RwSignal<CheckedState>,

    /// Whether the checkbox is disabled.
    #[prop(default = false.into(), into)]
    disabled: Signal<bool>,

    /// Form field name.
    #[prop(optional, into)]
    name: Option<String>,

    /// Form field value when checked.
    #[prop(optional, into)]
    value: Option<String>,

    /// Accessible label.
    #[prop(optional, into)]
    aria_label: Option<String>,

    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the root element.
    #[prop(optional, into)]
    style: Option<String>,

    /// HTML id attribute.
    #[prop(optional, into)]
    id: Option<String>,

    /// Reference to the button element.
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// The checkbox content (typically CheckboxIndicator).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = CheckboxContext { checked, disabled };
    provide_context(ctx);

    let on_click = move |_| {
        if !disabled.get() {
            checked.update(|c| {
                *c = match c {
                    CheckedState::Checked => CheckedState::Unchecked,
                    CheckedState::Unchecked | CheckedState::Indeterminate => CheckedState::Checked,
                };
            });
        }
    };

    let state_attr = move || checked.get().data_state();
    let aria_checked = move || checked.get().aria_checked();

    // Hidden input for form submission
    let input_value = value.clone();
    let input_name = name.clone();

    view! {
        <button
            node_ref=node_ref
            type="button"
            role="checkbox"
            id=id
            class=class
            style=style
            aria-label=aria_label
            aria-checked=aria_checked
            disabled=move || disabled.get()
            data-radix-checkbox-root=""
            data-state=state_attr
            data-disabled=move || disabled.get().then_some("")
            on:click=on_click
        >
            {children()}
        </button>
        {move || {
            // Render hidden input for form submission when checked
            if checked.get() == CheckedState::Checked {
                if let (Some(name), Some(value)) = (&input_name, &input_value) {
                    return Some(view! {
                        <input
                            type="hidden"
                            name=name.clone()
                            value=value.clone()
                        />
                    });
                }
            }
            None
        }}
    }
}

/// Indicator that renders only when checkbox is checked or indeterminate.
#[component]
pub fn CheckboxIndicator(
    /// CSS class name(s) for styling.
    #[prop(optional, into)]
    class: Option<String>,

    /// Inline styles for the indicator element.
    #[prop(optional, into)]
    style: Option<String>,

    /// Reference to the indicator element.
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Span>,

    /// The indicator content (checkmark, minus, etc.).
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = use_context::<CheckboxContext>()
        .expect("CheckboxIndicator must be used within CheckboxRoot");

    let state_attr = move || ctx.checked.get().data_state();
    let is_present = move || ctx.checked.get().is_present();

    view! {
        <Show when=is_present>
            <span
                node_ref=node_ref
                class=class.clone()
                style=style.clone()
                data-radix-checkbox-indicator=""
                data-state=state_attr
                data-disabled=move || ctx.disabled.get().then_some("")
            >
                {children()}
            </span>
        </Show>
    }
}
