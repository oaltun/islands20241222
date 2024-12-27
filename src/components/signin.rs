use leptos::prelude::*;
use oauth2::{CsrfToken, Scope};

use crate::{lib_ssr::pool::pool, structs::app_state::AppState};

#[server]
pub async fn google_sso() -> Result<String, ServerFnError> {
    let oauth_client = expect_context::<AppState>().client;

    let pool = pool()?;

    // We get the authorization URL and CSRF_TOKEN
    let (authorize_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // required for google auth refresh token to be part of the response.
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    let google_url = authorize_url.to_string();
    leptos::logging::log!("{google_url:?}");

    // Store the CSRF_TOKEN in our db.
    sqlx::query("INSERT INTO csrf_tokens (csrf_token) VALUES ($1)")
        .bind(csrf_token.secret())
        .execute(&pool)
        .await
        .map(|_| ())?;

    // Send the url to the client. Url points to a Google server.
    Ok(google_url)
}

#[component]
pub fn Signin() -> impl IntoView {
    view! {

        <div>
            <h1>"Signin Page"</h1>
            <p>"This page is only accessible via the /Signin route."</p>
        </div>
    }
}
