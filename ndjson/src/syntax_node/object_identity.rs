use crate::data_node::{StringParseError, StringToken, StringTokenizer};

pub struct ObjectIdentity {
	raw: String,
	escaped: Option<String>,
}

impl TryFrom<&str> for ObjectIdentity {
	type Error = StringParseError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let raw = value.to_string();

		if value == "" {
			Ok(Self { raw, escaped: None })
		} else {
			let mut tokenizer = StringTokenizer::new(value);
			let mut escaped = String::new();

			loop {
				if let Some(token) = tokenizer.next() {
					let token = token?;
					match token {
						StringToken::String(s) => escaped.push_str(s),
						StringToken::Char(c) => escaped.push(c),
					}
				} else {
					break;
				}
			}

			Ok(Self {
				raw,
				escaped: Some(escaped),
			})
		}
	}
}

impl ObjectIdentity {
	pub fn raw(&self) -> &str {
		&self.raw
	}

	pub fn escaped(&self) -> &str {
		match &self.escaped {
			Some(escaped) => escaped,
			None => self.raw.as_str(),
		}
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ObjectIdentity {
		pub fn assert_raw(&self, expected: &str) {
			assert_eq!(self.raw(), expected);
			assert_eq!(&self.raw, expected);
		}

		pub fn assert_escaped(&self, expected: &str) {
			assert_eq!(self.escaped(), expected);

			match &self.escaped {
				None => self.assert_raw(expected),
				Some(actual) => assert_eq!(actual, expected),
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_str() {
		let fixture = ObjectIdentity::try_from("hello world").unwrap();
		fixture.assert_raw("hello world");
		fixture.assert_escaped("hello world");

		let fixture = ObjectIdentity::try_from(r#"hello\nworld"#).unwrap();
		fixture.assert_raw(r#"hello\nworld"#);
		fixture.assert_escaped("hello\nworld");
	}

	#[test]
	fn try_from() {
		let fixture = ObjectIdentity::try_from("\\a").err().unwrap();
		fixture.assert_invalid_escape("\\a");

		let fixture = ObjectIdentity::try_from("\\").err().unwrap();
		fixture.assert_unexpected_eof();

		let fixture = ObjectIdentity::try_from(r#"ab\uDE0A\uD83E\uDEE0"#)
			.err()
			.unwrap();
		fixture.assert_invalid_surrogate("\\uDE0A", "");
	}
}
