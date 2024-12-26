use crate::components::app::App;
use crate::components::protected::Protected;
use crate::components::signin::Signin;
use leptos::prelude::*;
use leptos_router::components::*;
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
            <header>
            <nav>
                <a href="/">"Home"</a> "|"
                <a href="/signin">"Signin"</a> "|"
                <a href="/protected">"Protected"</a>
            </nav>
            </header>
                <Router>
                    <Routes fallback=|| view! { <h1>"Page Not Found"</h1> }>
                        <Route path=path!("/") view=|| view! { <App/> } />
                        <Route path=path!("/protected") view=|| view! { <Protected/> } />
                        <Route path=path!("/signin") view=|| view! { <Signin/> } />
                    </Routes>
                </Router>
            </body>
        </html>
    }
}
