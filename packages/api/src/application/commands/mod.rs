pub mod user {
    mod register_user_with_password;
    pub use register_user_with_password::*;
}

pub mod auth {
    mod password_authenticate;
    pub use password_authenticate::*;
}
