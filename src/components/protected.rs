use leptos::prelude::*;

#[component]
pub fn Protected() -> impl IntoView {
    view! {
        <header>
            <h1>"My Application"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/protected">"Protected"</a>
            </nav>
        </header>
        <div>
            <h1>"Protected PPage"</h1>
            <p>"This page is only accessible via the /protected route."</p>
        </div>
    }
}
