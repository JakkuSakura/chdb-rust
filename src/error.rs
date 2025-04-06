use std::ffi::NulError;
use std::fmt::Display;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An unknown error has occurred")]
    Unknown,
    #[error("No result")]
    NoResult,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Invalid path")]
    PathError,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Nul(#[from] NulError),
    #[error("Insufficient dir permissions")]
    InsufficientPermissions,
    #[error("Non UTF-8 sequence: {0}")]
    FromUtf8Error(FromUtf8Error),
    #[error("Invalid UTF-8 sequence: {0}")]
    InvalidUtf8Sequence(#[from] Utf8Error),
    #[error("Not enough data")]
    NotEnoughData,
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Invalid variant discriminator: {0}")]
    VariantDiscriminatorIsOutOfBound(usize),
    #[error("SequenceMustHaveLength")]
    SequenceMustHaveLength,
    #[error("DeserializeAnyNotSupported")]
    DeserializeAnyNotSupported,
    #[error("Invalid tag encoding: {0}")]
    InvalidTagEncoding(usize),
    #[error("Unsupported: {0}")]
    Unsupported(String),

    #[error("{0}")]
    QueryError(String),
}
impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerializationError(msg.to_string())
    }
}
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
pub type Result<T, Err = Error> = std::result::Result<T, Err>;
