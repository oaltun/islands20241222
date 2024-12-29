use leptos::prelude::*;

use crate::structs::app_state::AppState;

#[component]
pub fn Protected() -> impl IntoView {
    let mc = use_context::<String>().expect("no message");
    let message = mc;
    view! {

        <div>
            <h1>"Protected Page"</h1>
            <p>{message}</p>
            <p>"This page is only accessible via the /protected route."</p>
        </div>
    }
}
