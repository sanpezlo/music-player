use leptos::*;

mod control;
mod slider;
mod song;
mod volume;

pub use control::*;
pub use slider::*;
pub use song::*;
pub use volume::*;

#[component]
pub fn Player() -> impl IntoView {
    view! {
        <div class="flex h-full w-full flex-row items-center justify-between px-2">
            <PlayerSong/>

            <PlayerControl/>

            <PlayerVolume/>

        </div>
    }
}
