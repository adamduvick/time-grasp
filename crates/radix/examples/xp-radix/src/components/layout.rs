use leptos::prelude::*;
use leptos_router::components::{A, Outlet};

#[component]
pub fn Layout() -> impl IntoView {
    let primitives = crate::routes::primitives()
        .into_iter()
        .map(|p| view! { <li><A href=p.path attr:data-discover="true">{p.name}</A></li> })
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
