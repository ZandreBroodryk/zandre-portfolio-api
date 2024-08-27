use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use crate::{
    helpers::passwords::{hash_password, validate_password},
    web::Claims,
    Result, ServerError, KEYS,
};

#[derive(Clone)]
pub struct AuthController {
    pool: PgPool,
    secrets: SecretStore,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignUpPayload {
    pub username: String,
    pub password: String,
    pub display_name: String,
}

impl AuthController {
    pub fn new(pool: PgPool, secrets: SecretStore) -> Self {
        Self { pool, secrets }
    }
}

impl AuthController {
    pub async fn login(&self, payload: LoginPayload) -> Result<String> {
        let user_request =
            sqlx::query_file!("src/model/sql/get_user_by_email.sql", payload.username)
                .fetch_one(&self.pool)
                .await
                .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?;

        validate_password(payload.password, user_request.password_hash)?;

        let claims = Claims::new(user_request.id, payload.username);

        let token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| crate::ServerError::LoginFail)?;

        Ok(token)
    }

    pub async fn sign_up(&self, payload: SignUpPayload) -> Result<i32> {
        let password_hash = hash_password(payload.password, self.secrets.clone())?;
        let user_creation_request = sqlx::query_file!(
            "src/model/sql/add_user.sql",
            payload.username,
            password_hash,
            payload.display_name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?;

        return Ok(user_creation_request.id);
    }
}
