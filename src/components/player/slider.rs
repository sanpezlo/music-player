use leptos::*;

#[component]
pub fn Slider(value: ReadSignal<f64>) -> impl IntoView {
    let input_element = create_node_ref();

    create_effect(move |_| {
        let element: HtmlElement<html::Input> = input_element.get().unwrap();
        let _ = element.style("--slider-progress", format!("{}%", value.get() / 100.0));
    });

    view! {
        <input
            node_ref=input_element
            type="range"
            max="10000"
            class="
            cursor-pointer w-full bg-background-light relative
            appearance-none h-2 rounded-lg
            
            [&::-webkit-slider-thumb]:appearance-none
            [&::-webkit-slider-thumb]:hidden
            [&::-webkit-slider-thumb]:absolute
            [&::-webkit-slider-thumb]:bg-base-light
            [&::-webkit-slider-thumb]:w-4
            [&::-webkit-slider-thumb]:h-4
            [&::-webkit-slider-thumb]:rounded-full
            [&::-webkit-slider-thumb]:top-[-0.25rem]
            [&::-webkit-slider-thumb]:left-[calc(var(--slider-progress)-0.5rem)]
            [&:hover::-webkit-slider-thumb]:block
            [&:active::-webkit-slider-thumb]:block
            
            before:bg-base-light before:h-2 before:rounded-lg
            before:content-[''] before:absolute before:left-0
            before:right-[calc(100%-var(--slider-progress))]
            hover:before:bg-primary active:before:bg-primary
            "
            prop:value=move || value.get()
        />
    }
}
