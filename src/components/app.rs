use leptos::prelude::*;

use crate::components::inner_island::InnerIsland;
use crate::components::outer_island::OuterIsland;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <header>
            <h1>"My Application"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/protected">"Protected"</a>
            </nav>
        </header>
        <main>
            <OuterIsland>
                <InnerIsland/>
                <InnerIsland/>
                <InnerIsland/>
            </OuterIsland>
        </main>
    }
}
