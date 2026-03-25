//! Placeholder authentication for the Tolvex package registry

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub users: HashMap<String, User>,
    pub tokens: HashMap<String, String>, // token -> user_id
}

impl Default for AuthState {
    fn default() -> Self {
        let mut users = HashMap::new();
        users.insert(
            "alice".to_string(),
            User {
                id: "1".to_string(),
                username: "alice".to_string(),
                email: "alice@example.com".to_string(),
                is_admin: true,
            },
        );
        users.insert(
            "bob".to_string(),
            User {
                id: "2".to_string(),
                username: "bob".to_string(),
                email: "bob@example.com".to_string(),
                is_admin: false,
            },
        );

        let mut tokens = HashMap::new();
        tokens.insert("stub-token-123".to_string(), "1".to_string());
        tokens.insert("stub-token-456".to_string(), "2".to_string());

        Self { users, tokens }
    }
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    let token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = state.tokens.get(token).ok_or(StatusCode::UNAUTHORIZED)?;
    let user = state.users.get(user_id).ok_or(StatusCode::UNAUTHORIZED)?;

    // Store user in request extensions for downstream handlers
    let mut req = request;
    req.extensions_mut().insert(user.clone());

    Ok(next.run(req).await)
}

pub async fn require_admin(request: Request, next: Next) -> Result<Response, StatusCode> {
    let user = request
        .extensions()
        .get::<User>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if user.is_admin {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
