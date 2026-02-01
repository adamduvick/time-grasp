use leptos::prelude::*;
use radix::{AvatarFallback, AvatarImage, AvatarRoot};

#[component]
pub fn AvatarExample() -> impl IntoView {
    view! {
        <h1>"Avatar"</h1>
        <p>
            "User avatar with image and fallback support. Radix handles image loading "
            "states and fallback timing. User styles the appearance."
        </p>

        // Basic Avatar with Image
        <div class="example-section">
            <h2>"Basic Avatar with Image"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=1"
                        alt="User avatar"
                    />
                    <AvatarFallback class="avatar-fallback">"JD"</AvatarFallback>
                </AvatarRoot>
                <span>"John Doe"</span>
            </div>
        </div>

        // Fallback (No Image)
        <div class="example-section">
            <h2>"Fallback (No Image)"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root">
                    <AvatarFallback class="avatar-fallback">"AB"</AvatarFallback>
                </AvatarRoot>
                <span>"Alice Brown (no image provided)"</span>
            </div>
        </div>

        // Fallback on Error
        <div class="example-section">
            <h2>"Fallback on Error"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root">
                    <AvatarImage
                        class="avatar-image"
                        src="https://broken-url.example/image.jpg"
                        alt="User avatar"
                    />
                    <AvatarFallback class="avatar-fallback">"ER"</AvatarFallback>
                </AvatarRoot>
                <span>"Error Recovery (broken image URL)"</span>
            </div>
        </div>

        // Different Sizes
        <div class="example-section">
            <h2>"Different Sizes"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root" style="width: 32px; height: 32px">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=2"
                        alt="Small avatar"
                    />
                    <AvatarFallback class="avatar-fallback" style="font-size: 0.75rem">
                        "SM"
                    </AvatarFallback>
                </AvatarRoot>

                <AvatarRoot class="avatar-root" style="width: 48px; height: 48px">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=3"
                        alt="Medium avatar"
                    />
                    <AvatarFallback class="avatar-fallback">"MD"</AvatarFallback>
                </AvatarRoot>

                <AvatarRoot class="avatar-root" style="width: 64px; height: 64px">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=4"
                        alt="Large avatar"
                    />
                    <AvatarFallback class="avatar-fallback" style="font-size: 1.25rem">
                        "LG"
                    </AvatarFallback>
                </AvatarRoot>

                <AvatarRoot class="avatar-root" style="width: 96px; height: 96px">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=5"
                        alt="Extra large avatar"
                    />
                    <AvatarFallback class="avatar-fallback" style="font-size: 1.5rem">
                        "XL"
                    </AvatarFallback>
                </AvatarRoot>
            </div>
        </div>

        // Avatar Group
        <div class="example-section">
            <h2>"Avatar Group"</h2>
            <div style="display: flex">
                <AvatarRoot class="avatar-root" style="border: 2px solid var(--color-surface)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=11"
                        alt="User 1"
                    />
                    <AvatarFallback class="avatar-fallback">"U1"</AvatarFallback>
                </AvatarRoot>
                <AvatarRoot class="avatar-root" style="margin-left: -12px; border: 2px solid var(--color-surface)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=12"
                        alt="User 2"
                    />
                    <AvatarFallback class="avatar-fallback">"U2"</AvatarFallback>
                </AvatarRoot>
                <AvatarRoot class="avatar-root" style="margin-left: -12px; border: 2px solid var(--color-surface)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=13"
                        alt="User 3"
                    />
                    <AvatarFallback class="avatar-fallback">"U3"</AvatarFallback>
                </AvatarRoot>
                <AvatarRoot class="avatar-root" style="margin-left: -12px; border: 2px solid var(--color-surface)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=14"
                        alt="User 4"
                    />
                    <AvatarFallback class="avatar-fallback">"U4"</AvatarFallback>
                </AvatarRoot>
                <AvatarRoot class="avatar-root" style="margin-left: -12px; border: 2px solid var(--color-surface)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=15"
                        alt="User 5"
                    />
                    <AvatarFallback class="avatar-fallback">"U5"</AvatarFallback>
                </AvatarRoot>
                <AvatarRoot class="avatar-root" style="margin-left: -12px; border: 2px solid var(--color-surface); background: var(--color-border)">
                    <AvatarFallback class="avatar-fallback">"+3"</AvatarFallback>
                </AvatarRoot>
            </div>
        </div>

        // With Delay (Avoid Fallback Flash)
        <div class="example-section">
            <h2>"With Delay (Avoid Fallback Flash)"</h2>
            <p style="font-size: 0.875rem; margin-bottom: 1rem">
                "Use delay_ms on Fallback to avoid showing it briefly while image loads."
            </p>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=20"
                        alt="User avatar"
                    />
                    <AvatarFallback class="avatar-fallback" delay_ms=600>
                        "DL"
                    </AvatarFallback>
                </AvatarRoot>
                <span>"Fallback delayed by 600ms"</span>
            </div>
        </div>

        // Icon Fallback
        <div class="example-section">
            <h2>"Icon Fallback"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root">
                    <AvatarFallback class="avatar-fallback">
                        <svg
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
                            <circle cx="12" cy="7" r="4" />
                        </svg>
                    </AvatarFallback>
                </AvatarRoot>
                <span>"Icon instead of initials"</span>
            </div>
        </div>

        // Squared Avatar
        <div class="example-section">
            <h2>"Squared Avatar"</h2>
            <div style="display: flex; gap: 1rem; align-items: center">
                <AvatarRoot class="avatar-root" style="border-radius: var(--radius)">
                    <AvatarImage
                        class="avatar-image"
                        src="https://i.pravatar.cc/150?img=25"
                        alt="User avatar"
                        style="border-radius: var(--radius)"
                    />
                    <AvatarFallback class="avatar-fallback" style="border-radius: var(--radius)">
                        "SQ"
                    </AvatarFallback>
                </AvatarRoot>
                <span>"Rounded corners instead of circle"</span>
            </div>
        </div>

        // Data Attributes
        <div class="example-section">
            <h2>"Data Attributes"</h2>
            <ul style="font-size: 0.875rem; line-height: 1.8">
                <li>
                    <code>"data-state"</code>" on Image: \"loading\" | \"loaded\" | \"error\""
                </li>
            </ul>
            <p style="font-size: 0.875rem; margin-top: 1rem">
                "Fallback only renders when no Image is provided, Image is loading "
                "(after delay), or Image errors."
            </p>
        </div>
    }
}
