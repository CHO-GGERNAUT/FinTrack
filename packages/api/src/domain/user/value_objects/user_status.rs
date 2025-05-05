#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UserStatus {
    PendingActivation,
    Active,
    Inactive,
}
