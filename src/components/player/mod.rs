use leptos::*;
use slider::*;

mod slider;

#[component]
pub fn Player() -> impl IntoView {
    use crate::icons::*;

    let (is_playing, set_is_playing) = create_signal(false);

    let (volume_slider, set_volume_slider) = create_signal(10000.0);
    let (value_slider, set_value_slider) = create_signal(0.0);

    let volume = Signal::derive(move || (volume_slider.get() / 100.0) as i32);

    let prev_volume = store_value(volume.get_untracked());

    view! {
        <div class="flex h-full w-full flex-row items-center justify-between px-2">
            <div class="flex max-w-[200px] flex-1 items-center justify-center"></div>

            <div class="flex flex-1 items-center justify-center gap-4">
                <div class="flex max-w-[700px] flex-1 flex-col items-center justify-center">
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
            </div>

            <div class="flex max-w-[200px] flex-1 items-center justify-center">
                <div class="flex w-full max-w-32 justify-center gap-x-2 text-white items-center">
                    <button
                        class="opacity-70 transition hover:opacity-100"
                        on:click=move |_| {
                            let volume = volume.get_untracked();
                            if volume == 0 {
                                set_volume_slider.set(prev_volume.get_value() as f64 * 100.0);
                            } else {
                                prev_volume.set_value(volume);
                                set_volume_slider.set(0.0);
                            }
                        }
                    >

                        {move || {
                            match volume.get() {
                                0 => view! { <VolumeXIcon/> },
                                1..=33 => view! { <VolumeIcon/> },
                                34..=66 => view! { <Volume1Icon/> },
                                _ => view! { <Volume2Icon/> },
                            }
                        }}

                    </button>

                    <Slider
                        on:input=move |ev| {
                            let new_value: f64 = event_target_value(&ev).parse().unwrap();
                            set_volume_slider
                                .update(|volume| {
                                    *volume = new_value;
                                });
                        }

                        value=volume_slider
                    />

                </div>
            </div>

        </div>
    }
}
