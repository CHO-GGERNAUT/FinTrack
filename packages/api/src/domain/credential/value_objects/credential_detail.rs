use super::password::Password;

#[derive(Clone, Debug)] // 필요에 따라 PartialEq 등 추가
pub enum CredentialDetail {
    Password(Password),
}
