#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserStatus {
    PendingActivation,
    Active,
    Inactive,
    Suspended,
}
