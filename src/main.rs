use leptos::{
    component,
    mount::mount_to_body,
    prelude::{signal, ElementChild, Get, OnAttribute, Write},
    view, IntoView,
};

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        // {count} is a signal
        // 'count.get' get the inner value of the signal
        // 'set_count.set()' is used to set a fixed value, '.write()' updates the value 'in-place'
        <button on:click=move |_| *set_count.write() += 1>"Click me: " {count}</button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
