use leptos::*;

use crate::components::*;
use crate::icons::*;

#[component]
pub fn PlayerControl() -> impl IntoView {
    let (is_playing, set_is_playing) = create_signal(false);

    let (value_slider, set_value_slider) = create_signal(0.0);

    view! {
        <div class="flex flex-1 items-center justify-center max-w-[700px] flex-col">

            <div class="flex gap-4">
                <button class="transition hover:text-base-light">
                    <ShuffleIcon/>
                </button>

                <button class="transition hover:text-base-light">
                    <BackIcon/>
                </button>

                <button
                    class="rounded-full p-2 transition bg-base-light text-background scale-100 hover:scale-110"
                    on:click=move |_| {
                        set_is_playing.update(|is_playing| *is_playing = !*is_playing);
                    }
                >

                    {move || {
                        if is_playing.get() {
                            view! { <PauseIcon/> }
                        } else {
                            view! { <PlayIcon/> }
                        }
                    }}

                </button>

                <button class="transition hover:text-base-light">
                    <NextIcon/>
                </button>

                <button class="transition hover:text-base-light">
                    <RepeatIcon/>
                </button>
            </div>

            <div class="flex w-full gap-x-3 pt-2 text-xs items-center">
                <span class="w-12 text-right opacity-50">{"0:00"}</span>

                <Slider
                    on:input=move |ev| {
                        let new_value: f64 = event_target_value(&ev).parse().unwrap();
                        set_value_slider
                            .update(|value| {
                                *value = new_value;
                            });
                    }

                    on:change=move |_| {
                        logging::log!("change");
                    }

                    value=value_slider
                />

                <span class="w-12 opacity-50">{"0:00"}</span>
            </div>

        </div>
    }
}
