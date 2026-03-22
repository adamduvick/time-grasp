use leptos::prelude::*;
use cardo_ui::{AvatarFallback, AvatarImage, AvatarRoot};

#[component]
pub fn AvatarExample() -> impl IntoView {
    view! {
        <h2>"Avatar Primitive"</h2>

        <section style:margin-bottom="32px">
            <h3>"Basic Avatar with Image"</h3>
            <p>"Avatar displays an image when loaded successfully."</p>

            <AvatarRoot>
                <AvatarImage
                    src="https://picsum.photos/id/64/100/100"
                    alt="User avatar"
                />
                <AvatarFallback>"JD"</AvatarFallback>
            </AvatarRoot>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Fallback with Initials"</h3>
            <p>"When image fails to load, the fallback content is shown."</p>

            <AvatarRoot>
                <AvatarImage
                    src="https://invalid-url-that-will-fail.example/image.jpg"
                    alt="User avatar"
                />
                <AvatarFallback>"AB"</AvatarFallback>
            </AvatarRoot>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Fallback with Icon"</h3>
            <p>"Fallback can contain any content, such as an icon."</p>

            <AvatarRoot>
                <AvatarImage
                    src="https://another-invalid-url.example/image.jpg"
                    alt="User avatar"
                />
                <AvatarFallback>
                    <svg
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <circle cx="12" cy="8" r="4" />
                        <path d="M4 20c0-4 4-6 8-6s8 2 8 6" />
                    </svg>
                </AvatarFallback>
            </AvatarRoot>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Delayed Fallback"</h3>
            <p>"Fallback with a 500ms delay prevents flash on fast-loading images."</p>

            <div style:display="flex" style:gap="16px" style:align-items="center">
                <div>
                    <p style:margin-bottom="8px">"Fast image (no flash)"</p>
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/65/100/100"
                            alt="User avatar"
                        />
                        <AvatarFallback delay_ms=500>"FL"</AvatarFallback>
                    </AvatarRoot>
                </div>

                <div>
                    <p style:margin-bottom="8px">"Broken image (shows after delay)"</p>
                    <AvatarRoot>
                        <AvatarImage
                            src="https://broken-url.example/image.jpg"
                            alt="User avatar"
                        />
                        <AvatarFallback delay_ms=500>"DL"</AvatarFallback>
                    </AvatarRoot>
                </div>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Multiple Sizes"</h3>
            <p>"Avatar size can be customized with CSS."</p>

            <div style:display="flex" style:gap="16px" style:align-items="center">
                <div style:width="32px" style:height="32px">
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/66/100/100"
                            alt="Small avatar"
                        />
                        <AvatarFallback>"S"</AvatarFallback>
                    </AvatarRoot>
                </div>

                <AvatarRoot>
                    <AvatarImage
                        src="https://picsum.photos/id/67/100/100"
                        alt="Medium avatar"
                    />
                    <AvatarFallback>"M"</AvatarFallback>
                </AvatarRoot>

                <div style:width="64px" style:height="64px">
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/68/100/100"
                            alt="Large avatar"
                        />
                        <AvatarFallback>"L"</AvatarFallback>
                    </AvatarRoot>
                </div>
            </div>
        </section>

        <section style:margin-bottom="32px">
            <h3>"Avatar Group"</h3>
            <p>"Avatars can be stacked for group displays."</p>

            <div style:display="flex">
                <div style:margin-right="-12px" style:z-index="4">
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/69/100/100"
                            alt="User 1"
                        />
                        <AvatarFallback>"U1"</AvatarFallback>
                    </AvatarRoot>
                </div>
                <div style:margin-right="-12px" style:z-index="3">
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/70/100/100"
                            alt="User 2"
                        />
                        <AvatarFallback>"U2"</AvatarFallback>
                    </AvatarRoot>
                </div>
                <div style:margin-right="-12px" style:z-index="2">
                    <AvatarRoot>
                        <AvatarImage
                            src="https://picsum.photos/id/71/100/100"
                            alt="User 3"
                        />
                        <AvatarFallback>"U3"</AvatarFallback>
                    </AvatarRoot>
                </div>
                <div style:z-index="1">
                    <AvatarRoot>
                        <AvatarFallback>"+2"</AvatarFallback>
                    </AvatarRoot>
                </div>
            </div>
        </section>
    }
}
