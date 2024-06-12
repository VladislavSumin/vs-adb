use std::num::ParseIntError;
use std::str::Utf8Error;
use thiserror::Error;
use crate::adb_response_code::AdbResponseCode;

pub type AdbResult<T> = Result<T, AdbError>;

/// Represents all errors for this lib.
#[derive(Debug, Error)]
pub enum AdbError {
    #[error("Adb server return unknown code: {0}")]
    UnknownAdbResponseCodeError(String),
    #[error("Adb server return not valid response: {0:?}")]
    AdbResponseCodeParseError([u8; 4]),
    #[error("Adb server return error, code={0:?}, msg={1}")]
    AdbServerError(AdbResponseCode, String),

    #[error(transparent)]
    IoError(#[from]tokio::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
}
