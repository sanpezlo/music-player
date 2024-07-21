use chrono::*;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let hour = chrono::Local::now().hour();

    let greeting = if hour < 12 {
        "Good Morning"
    } else if hour < 18 {
        "Good Afternoon"
    } else {
        "Good Evening"
    };

    view! {
        <div class="flex-1 relative z-10 h-full px-6 pt-10 overflow-y-auto">
            <div class="absolute inset-0 -z-10 h-[calc((100vh-6rem)/2)] bg-gradient-to-t from-primary to-secondary h-"></div>

            <h1 class="mb-6 text-4xl font-bold text-base-light">{greeting}</h1>

        </div>
    }
}
