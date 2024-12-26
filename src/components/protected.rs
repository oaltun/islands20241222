use leptos::prelude::*;

#[component]
pub fn Protected() -> impl IntoView {
    view! {

        <div>
            <h1>"Protected PPage"</h1>
            <p>"This page is only accessible via the /protected route."</p>
        </div>
    }
}
