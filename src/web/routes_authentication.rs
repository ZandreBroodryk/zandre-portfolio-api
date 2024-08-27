use crate::model::{
    auth::{AuthController, LoginPayload, SignUpPayload},
    ApiState,
};
use axum::{extract::State, routing::post, Json, Router};
use serde_json::Value;

use crate::Result;

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .route("/signup", post(api_sign_up))
        .with_state(state)
}

async fn api_login(
    State(auth_controller): State<AuthController>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    let jwt_token = auth_controller.login(payload.clone()).await?;

    return Ok(Json(Value::String(jwt_token)));
}

async fn api_sign_up(
    State(auth_controller): State<AuthController>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Json<Value>> {
    let user_id = auth_controller.sign_up(payload).await?;

    return Ok(Json(Value::Number(user_id.into())));
}
