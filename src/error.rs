use base64::DecodeError;
use sha2::digest::crypto_common::InvalidLength;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug)]
pub enum Error {
    Base64Decode(DecodeError),
    InvalidLength(InvalidLength),
    // TODO: this is kind of lazy. It will include deserialization errors as well has whatever request provides. We probably also want to have true http errors (eg. 400 and 500)
    HttpError(reqwest::Error),
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

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e)
    }
}