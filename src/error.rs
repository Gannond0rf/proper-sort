use std::fmt::Display;

use derive_more::derive::From;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug, From)]
pub enum Error {
	Internal(String),
	RustDecimal(rust_decimal::Error),
	TokenNotSize,
	TokenNotNumber,
	TokenNotWhitespace,
	NoError,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error { }

impl From<&str> for Error {
	fn from(value: &str) -> Self {
		Self::Internal(value.to_string())
	}
}