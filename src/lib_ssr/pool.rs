use leptos::prelude::use_context;
use server_fn::ServerFnError;
use sqlx::PgPool;

pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool missing."))
}
