use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PasswordCredentialId(Uuid);

impl PasswordCredentialId {
    pub fn new() -> Self {
        PasswordCredentialId(Uuid::new_v4())
    }

    pub fn as_deref(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for PasswordCredentialId {
    fn from(uuid: Uuid) -> Self {
        PasswordCredentialId(uuid)
    }
}

impl From<PasswordCredentialId> for Uuid {
    fn from(credential_id: PasswordCredentialId) -> Self {
        credential_id.0
    }
}
