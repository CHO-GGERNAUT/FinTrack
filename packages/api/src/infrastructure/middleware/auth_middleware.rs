use std::sync::Arc;

use axum::{
    Extension,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;

use crate::application::{dto::Claims, services::JwtService};

pub async fn auth_middleware(
    Extension(jwt): Extension<Arc<dyn JwtService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    tracing::debug!("Auth middleware triggered");
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
        if let Ok(token_data) = jwt.verify(&token) {
            tracing::debug!("Token verified: {:?}", token_data.claims);
            request.extensions_mut().insert(Some(token_data.claims));
            return Ok(next.run(request).await);
        }
    }
    tracing::debug!("Token verification failed");
    request.extensions_mut().insert(None::<Claims>);
    Ok(next.run(request).await)
}
