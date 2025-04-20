#[derive(Debug)]
pub enum UserError {
    EmailAlreadyExists,
    UserNotFound,
    InvalidPassword,
    InvalidUserStatus,
    UserNotCreated,
}
