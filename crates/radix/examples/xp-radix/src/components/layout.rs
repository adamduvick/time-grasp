use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn Layout() -> impl IntoView {
    let primitives = crate::routes::primitives()
        .into_iter()
        .map(|p| {
            if p.implemented {
                view! { <li><A href=p.path attr:data-discover="true">{p.name}</A></li> }.into_any()
            } else {
                view! { <li><span class="nav-disabled">{p.name}</span></li> }.into_any()
            }
        })
        .collect_view();

    view! {
        <div class="layout">
        <nav class="sidebar">
            <h1>Radix Primitives</h1>
            <ul>
                <li>
                    <A href="/">"Home"</A>
                </li>
                {primitives}
            </ul>
        </nav>
        <main class="content">
            <Outlet />
        </main>
        </div>
    }
}
