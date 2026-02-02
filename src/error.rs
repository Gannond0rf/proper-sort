use std::fmt::Display;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug)]
pub enum Error {
	TokenNotSize,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error { }
