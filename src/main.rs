use axum::{middleware, Router};
use context::Keys;
use error::response_error_mapper;
use model::{auth::AuthController, blog::BlogController};
use once_cell::sync::Lazy;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

pub use self::error::{Result, ServerError};

mod context;
mod error;
mod helpers;
mod model;
mod web;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var(
        "JWT_SECRET",
        secrets.get("JWT_SECRET").expect("JWT_SECRET must be set"),
    );

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let blog_controller = BlogController::new(pool.clone());
    let auth_controller = AuthController::new(pool.clone(), secrets);

    let state = model::ApiState {
        blog_controller,
        auth_controller,
    };

    let router = Router::new()
        .nest("/api", web::api_routes(state))
        .layer(middleware::map_response(response_error_mapper));

    Ok(router.into())
}
