use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};

use crate::{web::Claims, Result, ServerError, KEYS};

#[derive(Debug, Clone)]
pub struct Context {
    user_id: i32,
}

impl Context {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl Context {
    pub fn current_user_id(&self) -> i32 {
        self.user_id
    }
}

pub async fn middleware_ctx_resolver(
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Decode the user data
    let context_result = match bearer {
        Some(TypedHeader(Authorization(bearer))) => {
            let token_data =
                decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
                    .map_err(|_| ServerError::AuthFailInvalidJwtToken)?;

            let context = Context::new(token_data.claims.get_user_id());
            Ok(context)
        }
        None => Err(ServerError::AuthFailNoAuthToken),
    };
    // if let Some(TypedHeader(Authorization(bearer))) = bearer {
    //     let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
    //         .map_err(|_| ServerError::AuthFailTokenWrongFormat)?;

    //     let context = Context::new(token_data.claims.get_user_id());
    //     // Store the ctx_result in the request extension.
    //     req.extensions_mut().insert(Ok(context));
    // }
    req.extensions_mut().insert(context_result);
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Result<Context>>()
            .ok_or(ServerError::AuthFailContextNotInRequest)?
            .clone()
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
