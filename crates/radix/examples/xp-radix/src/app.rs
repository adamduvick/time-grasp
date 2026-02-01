use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router_macro::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes transition=true fallback=|| "This page could not be found.">
                <ParentRoute path=path!("/") view=crate::components::Layout>
                    <Route path=path!("/") view=crate::routes::Home />
                    <Route path=path!("/accordion") view=crate::routes::AccordionExample />
                    <Route path=path!("/alert-dialog") view=crate::routes::AlertDialogExample />
                    <Route path=path!("/aspect-ratio") view=crate::routes::AspectRatioExample />
                    <Route path=path!("/avatar") view=crate::routes::AvatarExample />
                    <Route path=path!("/checkbox") view=crate::routes::CheckboxExample />
                    <Route path=path!("/collapsible") view=crate::routes::CollapsibleExample />
                    <Route path=path!("/context-menu") view=crate::routes::ContextMenuExample />
                    <Route path=path!("/dialog") view=crate::routes::DialogExample />
                    <Route path=path!("/dropdown") view=crate::routes::DropdownExample />
                    <Route path=path!("/hover-card") view=crate::routes::HoverCardExample />
                    <Route path=path!("/menubar") view=crate::routes::MenubarExample />
                    <Route path=path!("/navigation-menu") view=crate::routes::NavigationMenuExample />
                    <Route path=path!("/popover") view=crate::routes::PopoverExample />
                    <Route path=path!("/progress") view=crate::routes::ProgressExample />
                    <Route path=path!("/radio-group") view=crate::routes::RadioGroupExample />
                    <Route path=path!("/scroll-area") view=crate::routes::ScrollAreaExample />
                    <Route path=path!("/select") view=crate::routes::SelectExample />
                    <Route path=path!("/separator") view=crate::routes::SeparatorExample />
                    <Route path=path!("/slider") view=crate::routes::SliderExample />
                    <Route path=path!("/switch") view=crate::routes::SwitchExample />
                    <Route path=path!("/tabs") view=crate::routes::TabsExample />
                    <Route path=path!("/toast") view=crate::routes::ToastExample />
                    <Route path=path!("/toggle") view=crate::routes::ToggleExample />
                    <Route path=path!("/toggle-group") view=crate::routes::ToggleGroupExample />
                    <Route path=path!("/toolbar") view=crate::routes::ToolbarExample />
                    <Route path=path!("/tooltip") view=crate::routes::TooltipExample />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
