use thiserror::Error;

#[derive(Error, Debug)]
pub enum StringParseError {
	#[error("{first},{second} was invalid surrogate pair.")]
	InvalidSurrogate { first: String, second: String },
	#[error("{0} was invalid escape")]
	InvalidEscape(String),
	#[error("Unexpected end of file.")]
	UnexpectedEof(),
	#[error("{0} is invalid unescaped character")]
	InvalidUnescaped(char),
}

impl StringParseError {
	pub fn invalid_surrogate(first: String, second: String) -> StringParseError {
		StringParseError::InvalidSurrogate { first, second }
	}

	pub fn invalid_escape(value: String) -> StringParseError {
		StringParseError::InvalidEscape(value)
	}

	pub fn unexpected_eof() -> StringParseError {
		StringParseError::UnexpectedEof()
	}

	pub fn invalid_unescaped(c: char) -> StringParseError {
		StringParseError::InvalidUnescaped(c)
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl StringParseError {
		pub fn assert_invalid_surrogate(&self, first: &str, second: &str) {
			match self {
				StringParseError::InvalidSurrogate {
					first: f,
					second: s,
				} => {
					assert_eq!(f, first);
					assert_eq!(s, second);
				}
				_ => panic!("Expected InvalidSurrogate, but got {:?}", self),
			}
		}

		pub fn assert_invalid_escape(&self, value: &str) {
			match self {
				StringParseError::InvalidEscape(v) => {
					assert_eq!(v, value);
				}
				_ => panic!("Expected InvalidEscape, but got {:?}", self),
			}
		}

		pub fn assert_unexpected_eof(&self) {
			let StringParseError::UnexpectedEof() = self else {
				unreachable!()
			};
		}

		pub fn assert_invalid_unescaped(&self, expected: char) {
			if let StringParseError::InvalidUnescaped(act) = self {
				assert_eq!(act, &expected);
			}
		}
	}
}
