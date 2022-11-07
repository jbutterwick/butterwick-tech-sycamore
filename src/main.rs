use sycamore::prelude::*;
use chrono::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());

    let displayed_name = || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    let day = create_signal(cx, String::new());
    let mut number = create_signal(cx, 0);

    let displayed_date = || {
        if day.get().is_empty() {        
            Utc::now().format("%Y-%m-%d").to_string()
        } else {
            day.get().as_ref().clone()
        }
    };

    let displayed_number = || { number.get().as_ref().clone() };

    view! { cx,
        div {
            h1 {
                "Hello "
                (displayed_name())
                "!"
            }
            
            p {
                (displayed_date())
            }

            h2 {
                (displayed_number())
            }

            input(placeholder="What is your name?", bind:value=name)
            br{}
            br{}
            input(type="date", bind:value=day)
            br{}
            br{}
            button(on:click=move |_| { number += 1 }) {
                "+ 1"
            }

            button(on:click=move |_| { number -= 1 }) {
                "- 1"
            }
        }
    }
}


fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}