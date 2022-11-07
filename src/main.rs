use sycamore::prelude::*;
use chrono::prelude::*;

#[component]
fn Counter<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);
    let reset = |_| state.set(0);
    view! { cx,
        div {
            h2 { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }
        }
    }
}

#[component]
fn Hello<G: Html>(cx: Scope) -> View<G> {

    let name = create_signal(cx, String::new());

    let displayed_name = || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    let day = create_signal(cx, String::new());

    let displayed_date = || {
        if day.get().is_empty() {
            Utc::now().format("%Y-%m-%d").to_string()
        } else {
            day.get().as_ref().clone()
        }
    };

    view! { cx,
        div {
            h1 {"Hello "(displayed_name())"!"}
            input(placeholder="What is your name?", bind:value=name)
            p {(displayed_date())}
            input(type="date", bind:value=day)
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {

    view! { cx,
        div {
            Hello {}
            Counter {}
        }
    }
}


fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}