use leptos::prelude::*;

use crate::structs::app_state::AppState;

#[component]
pub fn Protected() -> impl IntoView {
    let app_state = use_context::<AppState>().expect("AppState context was not provided!");
    view! {cx,

        <div>
            <h1>"Protected Page"</h1>
            <p>"This page is only accessible via the /protected route."</p>
        </div>
    }
}
