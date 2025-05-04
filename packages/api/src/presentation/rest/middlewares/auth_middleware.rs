use std::sync::Arc;

use axum::{
    Extension,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;

use crate::application::interfaces::services::{TokenService, token_service::Claims};

pub async fn auth_middleware(
    Extension(auth): Extension<Arc<dyn TokenService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut token = request
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
        if let Ok(token_data) = auth.verify_token(&token) {
            request.extensions_mut().insert(Some(token_data));
            return Ok(next.run(request).await);
        }
    }
    request.extensions_mut().insert(None::<Claims>);
    Ok(next.run(request).await)
}
