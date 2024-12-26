use leptos::prelude::*;

use crate::components::inner_island::InnerIsland;
use crate::components::outer_island::OuterIsland;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <OuterIsland>
                <InnerIsland/>
                <InnerIsland/>
                <InnerIsland/>
            </OuterIsland>
        </main>
    }
}
