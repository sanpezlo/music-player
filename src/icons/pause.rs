use leptos::*;

#[component]
pub fn PauseIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="currentColor"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-pause"
        >
            <rect x="14" y="4" width="4" height="16" rx="1"></rect>
            <rect x="6" y="4" width="4" height="16" rx="1"></rect>
        </svg>
    }
}
