use leptos::*;

#[component]
pub fn NextIcon() -> impl IntoView {
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
            class="lucide lucide-skip-forward"
        >
            <polygon points="5 4 15 12 5 20 5 4"></polygon>
            <line x1="19" x2="19" y1="5" y2="19"></line>
        </svg>
    }
}
