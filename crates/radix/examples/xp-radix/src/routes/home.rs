use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    let primitives = crate::routes::primitives()
        .into_iter()
        .map(|p| {
            if p.implemented {
                view! {
                    <A href=p.path attr:class="primitive-card">
                        <h2>{p.name}</h2>
                        <p>{p.description}</p>
                    </A>
                }.into_any()
            } else {
                view! {
                    <div class="primitive-card primitive-card-disabled">
                        <h2>{p.name}</h2>
                        <p>{p.description}</p>
                    </div>
                }.into_any()
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
