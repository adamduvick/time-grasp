use leptos::prelude::*;
use cardo_ui::{Separator, SeparatorOrientation};

#[component]
pub fn SeparatorExample() -> impl IntoView {
    view! {
        <style inner_html="
            [data-radix-separator][data-orientation='horizontal'] {
                height: 1px;
                width: 100%;
                background-color: #e0e0e0;
            }
            [data-radix-separator][data-orientation='vertical'] {
                height: 100%;
                width: 1px;
                background-color: #e0e0e0;
            }
        " />

        <h2>"Separator"</h2>

        <h3>"Horizontal Separator"</h3>
        <div style:max-width="300px">
            <p>"Content above the separator"</p>
            <Separator />
            <p>"Content below the separator"</p>
        </div>

        <h3>"Vertical Separator"</h3>
        <div style:display="flex" style:align-items="center" style:height="20px" style:gap="16px">
            <span>"Left"</span>
            <Separator orientation=SeparatorOrientation::Vertical />
            <span>"Right"</span>
        </div>

        <h3>"Decorative Separator"</h3>
        <div style:max-width="300px">
            <p>"This separator is purely decorative (no semantic meaning)."</p>
            <Separator decorative=true />
            <p>"Inspect the element to see role=\"none\" instead of role=\"separator\"."</p>
        </div>

        <h3>"Styled Separators"</h3>
        <style inner_html="
            .thick-separator[data-orientation='horizontal'] {
                height: 3px;
                background: linear-gradient(to right, #667eea, #764ba2);
                border-radius: 2px;
            }
            .dashed-separator[data-orientation='horizontal'] {
                height: 1px;
                background: repeating-linear-gradient(
                    to right,
                    #999 0,
                    #999 4px,
                    transparent 4px,
                    transparent 8px
                );
            }
        " />
        <div style:max-width="300px" style:display="flex" style:flex-direction="column" style:gap="16px">
            <div>
                <p style:margin-bottom="8px">"Gradient separator:"</p>
                <Separator class="thick-separator" />
            </div>
            <div>
                <p style:margin-bottom="8px">"Dashed separator:"</p>
                <Separator class="dashed-separator" />
            </div>
        </div>
    }
}
