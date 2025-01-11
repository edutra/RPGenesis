use leptos::prelude::*;
use stylers::style;

#[component]
pub fn MyButton() -> impl IntoView {
    let (count, set_count) = signal(0);
    let style_class = style! { "MyButton",
        "background-color": "red",
        "color": "white",
        "padding": "10px",
        "margin": "10px",
        "border-radius": "5px",
        "cursor": "pointer",
    };

    fn minus(set_count: &WriteSignal<i32>) {
        *set_count.write() -= 1;
    }
    fn plus(set_count: &WriteSignal<i32>) {
        *set_count.write() += 1;
    }

    view! {
    <button
            on:click=move |_| {
                plus(&set_count);
            }
        >
            {move || if count.get() == 0 {
                "Click me".to_string()
            } else {
                count.get().to_string()
            }}
    </button>

    <button
            on:click=move|_| {
                minus(&set_count);
            }
        >
        {move|| if count.get() == 0 {
            "Click aqui".to_string()
        } else {
            count.get().to_string()
        }}

        </button>


    }
}

