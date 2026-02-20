use leptos::{
    component,
    prelude::{
        signal, ArcRwSignal, CollectView, ElementChild, For, Get, OnAttribute, RwSignal, Update,
        Write,
    },
    view, IntoView,
};

#[component]
pub fn StaticIter() -> impl IntoView {
    let values = vec![1, 2, 3, 4, 5];
    let signals: Vec<RwSignal<i32>> = values
        .clone()
        .into_iter()
        .map(|v| RwSignal::new(v))
        .collect();

    view! {
        <hr />
        <p>"Generating from static iterators"</p>
        // This will only plot the values as 12345
        <p>{values.clone()}</p>
        // collest_view get a Vec<View>
        <ul>{values.clone().into_iter().map(|v| view! { <li>{v}</li> }).collect_view()}</ul>

        {signals
            .into_iter()
            .map(|v| {
                view! { <button on:click=move |_| *v.write() += 1>"This is reactive! "{v}</button> }
            })
            .collect_view()}
    }
}

#[component]
pub fn DynamicIter() -> impl IntoView {
    let mut next_dyn_id = 4;
    let initial: Vec<(u32, ArcRwSignal<u32>)> = (0..next_dyn_id)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect();

    let (get_counters, set_counters) = signal(initial);

    let mut add_counter = move || {
        let sig = (next_dyn_id, ArcRwSignal::new(next_dyn_id + 1));
        set_counters.update(move |counters| counters.push(sig));
        next_dyn_id += 1;
    };

    view! {
        <hr />
        <p>"Generating from dynamic iterators"</p>
        <button on:click=move |_| add_counter()>"Add counter"</button>
        <For
            each=move || get_counters.get()
            key=|counter| counter.0
            children=move |(id, counter)| {
                let count = RwSignal::from(counter);
                view! {
                    <button on:click=move |_| *count.write() += 1>{count}</button>
                    <button on:click=move |_| {
                        set_counters.write().retain(|(counter_id, _)| counter_id != &id)
                    }>"Remove"</button>
                }
            }
        />
    }
}
