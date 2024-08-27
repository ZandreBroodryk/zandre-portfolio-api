use std::fmt::Display;

use axum::Router;
use chrono::{TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::model::ApiState;

pub mod routes_authentication;

pub mod routes_bog_posts;

pub fn api_routes(state: ApiState) -> Router {
    Router::new()
        .nest("/auth", routes_authentication::routes(state.clone()))
        .nest("/blogs", routes_bog_posts::routes(state.clone()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    id: i32,
    exp: i64,
}

impl Claims {
    pub fn new(id: i32, email: String) -> Self {
        let now = Utc::now().checked_add_signed(TimeDelta::minutes(30)).unwrap();
        Self {
            exp: now.timestamp(),
            sub: email,
            id: id,
        }
    }
}

impl Claims {
    pub fn get_user_id(&self) -> i32 {
        self.id
    }

    // pub fn get_user_email(&self) -> &str {
    //     &self.sub
    // }
}

impl Display for Claims {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(format, "Email: {}\nId: {}", self.sub, self.id)
    }
}
