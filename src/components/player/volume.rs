use leptos::*;

use crate::components::*;
use crate::icons::*;

#[component]
pub fn PlayerVolume() -> impl IntoView {
    let (volume_slider, set_volume_slider) = create_signal(10000.0);

    let volume = Signal::derive(move || (volume_slider.get() / 100.0) as i32);

    let prev_volume = store_value(volume.get_untracked());

    view! {
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
    }
}
