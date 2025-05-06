use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashingError {
    #[error("Password hashing failed: {0}")]
    HashingFailed(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Unsupported hashing algorithm or configuration: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Service internal error: {0}")] // 예상치 못한 서비스 내부 오류
    ServiceInternal(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    // ... 해싱 서비스에서 발생 가능한 다른 구체적 오류 variant들 ...
}

pub trait Hasher {
    fn hash(&self, plain_password: &str) -> Result<String, HashingError>;
}

pub trait Verifier {
    fn verify(&self, plain_password: &str, hash: &str) -> bool;
}
