use leptos::{
    component,
    control_flow::Show,
    logging,
    prelude::{signal, ElementChild, Get, OnAttribute, Write},
    view, IntoView,
};

// If and match control flows always rerender the inner component / view
#[component]
pub fn NativeIf() -> impl IntoView {
    let (value, set_value) = signal(0);

    view! {
        <button on:click=move |_| {
            *set_value.write() += 1;
        }>"Native If: "{value}- {move || if value.get() % 2 == 0 { "Even" } else { "Odd" }}</button>
    }
}

#[component]
pub fn Match() -> impl IntoView {
    let (value, set_value) = signal(0);

    view! {
        <button on:click=move |_| {
            *set_value.write() += 1;
        }>
            "Match stmt: "{value}-
            {move || {
                match value.get() {
                    0 => "Zero",
                    1 => "One",
                    n if n % 2 > 0 => "Odd",
                    _ => "Even",
                }
            }}
        </button>
    }
}

#[component]
pub fn LeptosShow() -> impl IntoView {
    let (value, set_value) = signal(0);

    // <Show> component memoizes the when value, so it only rerender when the expression value
    // change.
    // when => The expression to evaluate
    // fallback => view to render when the fallback is false
    view! {
        <button on:click=move |_| {
            *set_value.write() += 1;
        }>
            "Value: "{value}" Showing:"
            <Show when=move || { value.get() > 5 } fallback=|| view! { <Small /> }>
                <Big />
            </Show>
        </button>
    }
}

#[component]
fn Big() -> impl IntoView {
    logging::log!("Rendering Big component");
    view! {
        <div>
            <p>"Its big"</p>
        </div>
    }
}

#[component]
fn Small() -> impl IntoView {
    logging::log!("Rendering Small component");
    view! {
        <div>
            <p>"Its small"</p>
        </div>
    }
}
