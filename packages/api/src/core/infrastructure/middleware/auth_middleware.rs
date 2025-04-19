use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let mut token = None;
    token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });
    if token.is_none() {
        if let Some(cookie_header) = request.headers().get(header::COOKIE) {
            let cookie_str = cookie_header.to_str().unwrap_or("");
            for cookie in cookie_str.split(';') {
                if let Ok(cookie) = Cookie::parse(cookie.trim()) {
                    if cookie.name() == "auth_token" {
                        token = Some(cookie.value().to_string());
                        break;
                    }
                }
            }
        }
    }
    if let Some(token) = token {
        let config = Config::get();
        let jwt_secret = config.jwt_secret.as_bytes();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let claims = token_data.claims;

        request.extensions_mut().insert(Some(claims));
    }

    Ok(next.run(request).await)
}
