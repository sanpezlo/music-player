mod components;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

/// The main App component
#[component]
pub fn App() -> impl IntoView {
    use components::Nav;

    view! {
        <Router>
            <Body class="relative grid h-screen gap-2 bg-background p-2 [grid-template-areas:'aside_main_main''player_player_player'] [grid-template-columns:280px_1fr] [grid-template-rows:1fr_72px] text-base"/>

            <aside class="flex flex-col overflow-y-auto [grid-area:aside]">
                <Nav/>
            </aside>

            <main class="flex flex-col overflow-y-auto rounded-lg bg-primary [grid-area:main]">
                <Routes>
                    <Route path="/" view=|| view! {}/>
                </Routes>
            </main>

            <footer class="rounded-lg [grid-area:player]"></footer>
        </Router>
    }
}
