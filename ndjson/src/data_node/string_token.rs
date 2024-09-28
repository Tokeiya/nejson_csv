pub enum StringToken<'a> {
	String(&'a str),
	Char(char),
}

impl<'a> From<&'a str> for StringToken<'a> {
	fn from(value: &'a str) -> Self {
		StringToken::String(value)
	}
}

impl From<char> for StringToken<'_> {
	fn from(value: char) -> Self {
		StringToken::Char(value)
	}
}

#[cfg(test)]
pub mod test_helper {
	use crate::data_node::string_token::StringToken;

	impl StringToken<'_> {
		pub fn assert_string(&self, expected: &str) {
			let StringToken::String(act) = self else {
				unreachable!()
			};

			assert_eq!(*act, expected);
		}

		pub fn assert_char(&self, expected: char) {
			let StringToken::Char(act) = self else {
				unreachable!()
			};
			assert_eq!(act, &expected);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::test_helper;
	use super::*;
	#[test]
	fn test_string() {
		let fixture = StringToken::from("hello");
		fixture.assert_string("hello");
	}

	#[test]
	fn test_char() {
		let fixture = StringToken::Char('a');
		fixture.assert_char('a');
	}
}
