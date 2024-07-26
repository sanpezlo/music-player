use leptos::*;

#[component]
pub fn BackIcon() -> impl IntoView {
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
            class="lucide lucide-skip-back"
        >
            <polygon points="19 20 9 12 19 4 19 20"></polygon>
            <line x1="5" x2="5" y1="19" y2="5"></line>
        </svg>
    }
}
