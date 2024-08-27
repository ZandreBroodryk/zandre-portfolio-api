use auth::AuthController;
use axum::extract::FromRef;
use blog::BlogController;

pub mod auth;
pub mod blog;

#[derive(Clone, FromRef)]
pub struct ApiState {
    pub blog_controller: BlogController,
    pub auth_controller: AuthController,
}
