use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    let primitives = crate::routes::primitives()
        .into_iter()
        .map(|(route, title, description)| {
            view! {
                <A href=route attr:class="primitive-card">
                    <h2>{title}</h2>
                    <p>{description}</p>
                </A>

            }
        })
        .collect_view();

    view! {
      <h1>"Radix UI Primitives Explorer"</h1>
      <p>
        "This app demonstrates Radix UI primitives with minimal styling to
        understand the boundary between what Radix provides (behavior,
        accessibility, state) and what the user must implement (visual styling)."
      </p>
      <div class="primitive-grid">
        {primitives}
      </div>
    }
}
