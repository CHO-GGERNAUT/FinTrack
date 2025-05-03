use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CredentialId(Uuid);

impl CredentialId {
    pub fn new() -> Self {
        CredentialId(Uuid::new_v4())
    }
}

impl From<Uuid> for CredentialId {
    fn from(uuid: Uuid) -> Self {
        CredentialId(uuid)
    }
}
