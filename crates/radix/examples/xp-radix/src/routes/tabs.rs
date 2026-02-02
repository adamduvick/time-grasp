use leptos::prelude::*;
use radix::{TabsActivationMode, TabsContent, TabsList, TabsOrientation, TabsRoot, TabsTrigger};

/// Tabs Primitive
///
/// RADIX PROVIDES:
/// - Tab/panel association via value prop
/// - Keyboard navigation (arrow keys between tabs)
/// - data-state="active" | "inactive" on triggers
/// - data-state="active" | "inactive" on content
/// - data-orientation="horizontal" | "vertical"
/// - Proper ARIA: role="tablist", role="tab", role="tabpanel"
/// - activationMode: "automatic" (focus activates) or "manual" (enter/space activates)
///
/// USER MUST IMPLEMENT:
/// - Tab list styling (flex container, borders)
/// - Active tab indicator
/// - Tab panel styling
/// - Any animations

#[component]
pub fn TabsExample() -> impl IntoView {
    view! {
        <h1>"Tabs"</h1>
        <p>
            "Tabbed content panels. Radix handles tab/panel association, keyboard "
            "nav, and ARIA. User styles the tab list, indicators, and panels."
        </p>

        <div class="example-section">
            <h2>"Basic Tabs"</h2>
            <TabsRoot value=RwSignal::new("tab1".to_string())>
                <TabsList class="tabs-list">
                    <TabsTrigger class="tabs-trigger" value="tab1">
                        "Account"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="tab2">
                        "Documents"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="tab3">
                        "Settings"
                    </TabsTrigger>
                </TabsList>
                <TabsContent class="tabs-content" value="tab1">
                    <h3>"Account Settings"</h3>
                    <p>"Manage your account details and preferences here."</p>
                </TabsContent>
                <TabsContent class="tabs-content" value="tab2">
                    <h3>"Documents"</h3>
                    <p>"View and manage your uploaded documents."</p>
                </TabsContent>
                <TabsContent class="tabs-content" value="tab3">
                    <h3>"Settings"</h3>
                    <p>"Configure application settings."</p>
                </TabsContent>
            </TabsRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Try: Arrow keys to navigate between tabs"
            </p>
        </div>

        <div class="example-section">
            <h2>"Manual Activation (require Enter/Space)"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "With activationMode=\"manual\", arrow keys move focus but don't activate "
                "the tab. Press Enter or Space to activate."
            </p>
            <TabsRoot value=RwSignal::new("manual1".to_string()) activation_mode=TabsActivationMode::Manual>
                <TabsList class="tabs-list">
                    <TabsTrigger class="tabs-trigger" value="manual1">
                        "Tab 1"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="manual2">
                        "Tab 2"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="manual3">
                        "Tab 3"
                    </TabsTrigger>
                </TabsList>
                <TabsContent class="tabs-content" value="manual1">
                    "Content for Tab 1"
                </TabsContent>
                <TabsContent class="tabs-content" value="manual2">
                    "Content for Tab 2"
                </TabsContent>
                <TabsContent class="tabs-content" value="manual3">
                    "Content for Tab 3"
                </TabsContent>
            </TabsRoot>
        </div>

        <div class="example-section">
            <h2>"Vertical Orientation"</h2>
            <TabsRoot value=RwSignal::new("v1".to_string()) orientation=TabsOrientation::Vertical>
                <div style="display: flex; gap: 1rem">
                    <TabsList
                        class="tabs-list"
                        style="flex-direction: column; border-bottom: none; border-right: 1px solid var(--color-border)"
                    >
                        <TabsTrigger
                            class="tabs-trigger"
                            value="v1"
                            style="border-bottom: none; border-right: 2px solid transparent"
                        >
                            "Overview"
                        </TabsTrigger>
                        <TabsTrigger
                            class="tabs-trigger"
                            value="v2"
                            style="border-bottom: none; border-right: 2px solid transparent"
                        >
                            "Analytics"
                        </TabsTrigger>
                        <TabsTrigger
                            class="tabs-trigger"
                            value="v3"
                            style="border-bottom: none; border-right: 2px solid transparent"
                        >
                            "Reports"
                        </TabsTrigger>
                    </TabsList>
                    <div style="flex: 1">
                        <TabsContent class="tabs-content" value="v1">
                            <h3>"Overview"</h3>
                            <p>"Dashboard overview content."</p>
                        </TabsContent>
                        <TabsContent class="tabs-content" value="v2">
                            <h3>"Analytics"</h3>
                            <p>"Analytics and metrics content."</p>
                        </TabsContent>
                        <TabsContent class="tabs-content" value="v3">
                            <h3>"Reports"</h3>
                            <p>"Generated reports content."</p>
                        </TabsContent>
                    </div>
                </div>
            </TabsRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Note: Up/Down arrows navigate vertical tabs (not Left/Right)"
            </p>
        </div>

        <div class="example-section">
            <h2>"With Disabled Tab"</h2>
            <TabsRoot value=RwSignal::new("d1".to_string())>
                <TabsList class="tabs-list">
                    <TabsTrigger class="tabs-trigger" value="d1">
                        "Active"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="d2" disabled=true>
                        "Disabled"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="d3">
                        "Also Active"
                    </TabsTrigger>
                </TabsList>
                <TabsContent class="tabs-content" value="d1">
                    "First tab content"
                </TabsContent>
                <TabsContent class="tabs-content" value="d2">
                    "You shouldn't see this"
                </TabsContent>
                <TabsContent class="tabs-content" value="d3">
                    "Third tab content"
                </TabsContent>
            </TabsRoot>
            <p style="margin-top: 1rem; font-size: 0.875rem">
                "Keyboard navigation skips the disabled tab"
            </p>
        </div>

        <div class="example-section">
            <h2>"Dynamic Content"</h2>
            <TabsRoot value=RwSignal::new("code".to_string())>
                <TabsList class="tabs-list">
                    <TabsTrigger class="tabs-trigger" value="code">
                        "Code"
                    </TabsTrigger>
                    <TabsTrigger class="tabs-trigger" value="preview">
                        "Preview"
                    </TabsTrigger>
                </TabsList>
                <TabsContent class="tabs-content" value="code">
                    <pre style="background: var(--color-bg); padding: 1rem; border-radius: var(--radius); overflow: auto">
                        "function hello() {\n  console.log(\"Hello, World!\");\n}"
                    </pre>
                </TabsContent>
                <TabsContent class="tabs-content" value="preview">
                    <div style="background: var(--color-bg); padding: 1rem; border-radius: var(--radius)">
                        <p>"Output: Hello, World!"</p>
                    </div>
                </TabsContent>
            </TabsRoot>
        </div>
    }
}
