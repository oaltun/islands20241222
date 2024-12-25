use leptos::prelude::*;

#[island]
pub fn InnerIsland() -> impl IntoView {
    let val = use_context::<i32>();
    view! {
        <h2>"Inner Island"</h2>
        <button on:click=move |_| leptos::logging::log!("value should be Some(42) -- it's {val:?}")>
            "Click me (inner)"
        </button>
    }
}
