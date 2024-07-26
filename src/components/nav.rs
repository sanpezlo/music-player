use leptos::*;
use leptos_router::*;

/// A Nav component
#[component]
pub fn Nav() -> impl IntoView {
    use crate::icons::*;

    view! {
        <nav class="flex flex-1 flex-col gap-2 overflow-y-auto">
            <Ul class="p-2">
                <Item href="/">
                    <HomeIcon/>
                    Home
                </Item>
                <Item href="/">
                    <SearchIcon/>
                    Search
                </Item>
            </Ul>
            <Ul class="flex flex-1 flex-col overflow-y-auto">
                <Item href="/" class="m-2">
                    <LibraryIcon/>
                    Your library
                </Item>

                <div class="overflow-y-auto p-2"></div>
            </Ul>
        </nav>
    }
}

/// A Unordered List component
#[component]
fn Ul(
    /// Children
    children: Children,
    /// Optional class name
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    view! { <ul class=format!("rounded-lg bg-background {class}")>{children()}</ul> }
}

/// A Nav Item component
#[component]
fn Item(
    /// Children
    children: Children,
    /// href
    href: &'static str,
    /// Optional class name
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <li class=class>
            <A
                href=href
                class="flex items-center gap-4 px-5 py-3 font-medium transition duration-300 hover:text-base-light focus:text-base-light"
            >
                {children()}
            </A>

        </li>
    }
}
