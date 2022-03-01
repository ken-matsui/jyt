use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("A serialization error occurred: {0}")]
    Serialization(String),

    #[error("A deserialization error occurred: {0}")]
    Deserialization(String),
}
