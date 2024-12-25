use leptos::prelude::*;

#[island]
pub fn OuterIsland(children: Children) -> impl IntoView {
    provide_context(42i32);
    view! {
        <div class="outer-island">
            <h2>"Outer Island"</h2>
            <button on:click=|_| leptos::logging::log!("clicked button in island!")>
                "Click me"
            </button>
            {children()}
        </div>
    }
}
