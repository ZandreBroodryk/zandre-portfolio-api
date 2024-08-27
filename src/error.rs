use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Clone, Debug, Serialize)]
pub enum ServerError {
    LoginFail,

    AuthFailNoAuthToken,
    AuthFailInvalidJwtToken,
    AuthFailContextNotInRequest,
    AuthFailInvalidPassword,
    AuthFailNoAccount,
    // Database Errors
    DatabaseFailure(String),

    //ServerErrors
    NoSaltPhrase,
    SaltEncodingFail,
    HashFail(String)
}

pub enum ClientError {
    LoginFail,
    Unauthorized,
    NotFound,
    Forbidden,
    ServerError,
}

impl ServerError {
    pub fn map_to_client_error(&self) -> ClientError {
        match self {
            Self::LoginFail => ClientError::LoginFail,
            Self::AuthFailContextNotInRequest
            | Self::AuthFailNoAuthToken
            | Self::AuthFailInvalidJwtToken => ClientError::Unauthorized,
            Self::DatabaseFailure(..) => ClientError::NotFound,
            _ => ClientError::ServerError,
        }
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        let status_message = match self {
            ClientError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            ClientError::LoginFail => (StatusCode::FORBIDDEN, "Forbidden"),
            ClientError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ClientError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            ClientError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        status_message.into_response()
    }
}

pub async fn response_error_mapper(response: Response) -> Response {
    let service_error = response.extensions().get::<ServerError>();

    if let Some(service_error) = service_error {
        return service_error.map_to_client_error().into_response();
    }

    response
}
