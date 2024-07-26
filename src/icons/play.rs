use leptos::*;

#[component]
pub fn PlayIcon() -> impl IntoView {
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
            class="lucide lucide-play"
        >
            <polygon points="6 3 21 12 6 21 6 3"></polygon>
        </svg>
    }
}
