use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn Layout() -> impl IntoView {
    let primitives = crate::routes::primitives()
        .into_iter()
        .map(|(route, title, _)| view! { <li><A href=route attr:data-discover="true">{title}</A></li> })
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
