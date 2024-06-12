use thiserror::Error;

pub type AdbResult<T> = Result<T, AdbError>;

/// Represents all errors for this lib.
#[derive(Debug, Error)]
pub enum AdbError {
    #[error(transparent)]
    Io(#[from]tokio::io::Error),

    #[error("Adb server return unknown code: {0}")]
    UnknownAdbResponseCode(String),

    #[error("Adb server return not valid response: {0:?}")]
    AdbResponseCodeParseError([u8; 4]),
}
