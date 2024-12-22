use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options islands=true/>
                <link rel="stylesheet" id="leptos" href="/pkg/islands.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
            </head>
            <body>
                <Router>
                    <Routes fallback=|| view! { <h1>"Page Not Found"</h1> }>
                        <Route path=path!("/") view=|| view! { <App/> } />
                        <Route path=path!("/protected") view=|| view! { <Protected/> } />
                    </Routes>
                </Router>
            </body>
        </html>
    }
}

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
            <h1>"Protected Page"</h1>
            <p>"This page is only accessible via the /protected route."</p>
        </div>
    }
}

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
