use std::str::from_utf8;

use crate::adb_error::AdbError;

#[derive(Debug)]
pub enum AdbResponseCode {
    Okay,
    Fail,
}

impl TryFrom<&[u8; 4]> for AdbResponseCode {
    type Error = AdbError;

    fn try_from(value: &[u8; 4]) -> Result<Self, Self::Error> {
        match value {
            b"OKAY" => { Ok(Self::Okay) }
            b"FAIL" => { Ok(Self::Fail) }
            _ => {
                let str_response = from_utf8(value)
                    .map_err(move |_| { AdbError::AdbResponseCodeParseError(value.clone()) })?;
                Err(AdbError::UnknownAdbResponseCodeError(str_response.to_owned()))
            }
        }
    }
}