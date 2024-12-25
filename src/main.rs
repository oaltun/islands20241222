pub mod components;
pub mod structs;

use axum::Router;
use components::{app::App, shell::shell};
use leptos::prelude::*;
use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
use structs::app_state::AppState;

#[tokio::main]
async fn main() {
    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // We create our oauth2 client using provided environment variables.
    let client = oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(
            std::env::var("GOOGLE_AUTH_CLIENT_ID").expect("GOOGLE_AUTH_CLIENT Env var to be set."),
        ),
        Some(oauth2::ClientSecret::new(
            std::env::var("GOOGLE_AUTH_SECRET").expect("GOOGLE_AUTH_SECRET Env var to be set"),
        )),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(
        oauth2::RedirectUrl::new(
            std::env::var("GOOGLE_AUTH_REDIRECT_URL")
                .expect("GOOGLE_AUTH_REDIRECT_URL Env var to be set"),
        )
        .unwrap(),
    );

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        //pool: pool.clone(),
        client,
    };

    // build our application
    let app = Router::new()
        .leptos_routes(&app_state, routes, {
            // let app_state = app_state.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
