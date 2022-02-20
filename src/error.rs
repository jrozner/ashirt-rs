use base64::DecodeError;
use sha2::digest::crypto_common::InvalidLength;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Base64Decode(DecodeError),
    InvalidLength(InvalidLength),
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Self {
        Self::Base64Decode(e)
    }
}

impl From<InvalidLength> for Error {
    fn from(e: InvalidLength) -> Self {
        Self::InvalidLength(e)
    }
}
