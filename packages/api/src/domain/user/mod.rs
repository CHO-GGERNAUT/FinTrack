pub mod entities {
    mod user; // Aggregate root
    pub use user::User;
}

pub mod value_objects {
    mod user_id;
    pub use user_id::UserId;

    mod email;
    pub use email::Email;

    mod phone_number;
    pub use phone_number::PhoneNumber;

    mod user_status;
    pub use user_status::UserStatus;
}

pub mod errors;
