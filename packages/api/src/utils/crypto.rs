use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash};

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(Into::into)
}
