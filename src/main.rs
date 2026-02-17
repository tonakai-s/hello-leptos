use leptos::{
    component,
    mount::mount_to_body,
    prelude::{signal, ClassAttribute, ElementChild, Get, OnAttribute, StyleAttribute, Write},
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
        <button
            on:click=move |_| { *set_count.write() += 1 }

            // class:classname=bool(fixed bool or an expression that evaluates to bool)
            // some classes with dashes are not parsed correctly, these cases use a tuple:
            // class=("classname", bool)
            // for multiple classes in same expression can use an array:
            // class=(["classname1" , "classname2", "classname3"], bool)
            class:red=move || count.get() % 2 == 1

            // style attr
            style="position: absolute"
            // style also can have dynamic props
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // this sets a CSS variable
            style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>

        // pass a signal to a attribute make it also update reactively
        <progress max="50" value=count />
    }
}
