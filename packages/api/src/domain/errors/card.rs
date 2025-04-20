#[derive(Debug)]
pub enum CardError {
    InvalidCardNumber,
    DuplicateCard,
    ExceedsCreditLimit,
    CardNotFound,
    Unknown(String),
}
