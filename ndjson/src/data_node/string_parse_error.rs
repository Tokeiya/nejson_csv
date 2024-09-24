use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringParseError {
	#[error("{first},{second} was invalid surrogate pair.")]
	InvalidSurrogate { first: String, second: String },
	#[error("{0} was invalid escape")]
	InvalidEscape(String),
}

impl StringParseError {
	pub fn invalid_surrogate(first: String, second: String) -> StringParseError {
		StringParseError::InvalidSurrogate { first, second }
	}

	pub fn invalid_escape(value: String) -> StringParseError {
		StringParseError::InvalidEscape(value)
	}
}
