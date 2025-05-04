use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PasswordCredentialId(Uuid);

impl PasswordCredentialId {
    pub fn new() -> Self {
        PasswordCredentialId(Uuid::new_v4())
    }
}

impl From<Uuid> for PasswordCredentialId {
    fn from(uuid: Uuid) -> Self {
        PasswordCredentialId(uuid)
    }
}
