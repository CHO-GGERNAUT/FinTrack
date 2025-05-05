use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardId(Uuid);

impl CardId {
    pub fn new() -> Self {
        CardId(Uuid::new_v4())
    }

    pub fn as_deref(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CardId {
    fn from(uuid: Uuid) -> Self {
        CardId(uuid)
    }
}

impl From<CardId> for Uuid {
    fn from(user_id: CardId) -> Self {
        user_id.0
    }
}

impl std::fmt::Display for CardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
