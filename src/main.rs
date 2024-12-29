pub mod components;
pub mod lib_ssr;
pub mod structs;

use axum::Router;
use components::{app::App, shell::shell};
use leptos::prelude::*;
use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;
use structs::app_state::AppState;

#[tokio::main]
async fn main() {
    // Create a db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Could not create PostgreSQL connection pool.");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

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
        pool: pool.clone(),
        client,
        message: "the message".to_string(),
    };

    // Clone `app_state` so it can be moved into the closure
    let app_state_clone = app_state.clone();
    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            move || {
                // Provide additional context to the reactive system
                provide_context(app_state_clone.clone());
            },
            move || shell(leptos_options.clone()),
        )
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state.clone());

    provide_context(app_state.clone());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
