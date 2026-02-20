use hello_leptos::{
    components::{
        iter::{DynamicIter, StaticIter},
        progress::ProgressBar,
    },
    spread::SpreadingExample,
};

use leptos::{
    component,
    mount::mount_to_body,
    prelude::{
        signal, AddAnyAttr, ClassAttribute, ElementChild, Get, InnerHtmlAttribute, OnAttribute,
        StyleAttribute, Write,
    },
    view, IntoView,
};

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    // derived signal: this calc run everytime it's called. Since this is cheap, is ok.
    // Memo is a feature that solves this to expensive calculations.
    let double_count = move || count.get() * 2;

    let html = "<p>This HTML will be injected</p>";

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
            style="position: relative"
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

        <p>"Double count: " {double_count}</p>

        // props to components are passed like HTML attributes
        <ProgressBar value=count style:background-color="aqua" class:foo=true />
        <ProgressBar max=5 value=double_count />

        // inner_html wipes out all the childrens of the element.
        // the inserted HTML is not escaped, so is vulnerable to XSS
        <div inner_html=html></div>

        <SpreadingExample />

        <StaticIter />
        <DynamicIter />
    }
}
