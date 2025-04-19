use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,     // 토큰 소유자 (subject)
    pub user_id: String, // 내부 유저 식별자
    pub exp: usize,      // 만료 시간 (timestamp, 초 단위)
}
