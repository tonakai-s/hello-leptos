use leptos::{component, prelude::Signal, view, IntoView};

#[component]
// props must have type Signal to be reactive
// #[prop(default=N)] define a default value to the prop when none is given
// if used #[prop(optional)] the Default::default() value of the type will be used instead
pub fn ProgressBar(
    #[prop(default = 10)] max: u16,
    // Signal is a enum that supports Signal, Memos, RwSignal or Derived signals
    // #[prop(into)] automatically call .into() in the provided value for the prop
    #[prop(into)] value: Signal<i32>,
) -> impl IntoView {
    view! {
        // pass a signal to a attribute make it also update reactively
        <progress max=max value=value />
    }
}
