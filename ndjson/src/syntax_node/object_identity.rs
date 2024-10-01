use crate::data_node::{StringParseError, StringToken, StringTokenizer};

pub struct ObjectIdentity {
	raw: String,
	escaped: Option<String>,
}

impl From<&str> for ObjectIdentity {
	fn from(value: &str) -> Self {
		let raw = value.to_string();

		if value == "" {
			Self { raw, escaped: None }
		} else {
			let mut tokenizer = StringTokenizer::new(&raw);
			let mut escaped = String::new();

			loop {
				if let Some(token) = tokenizer.next() {
					match token {
						Ok(t) => match t {
							StringToken::String(s) => escaped.push_str(&s),
							StringToken::Char(c) => escaped.push(c),
						},
						Err(e) => {
							escaped.push_str(&e.to_string());
						}
					}
				} else {
					break;
				}
			}

			if &escaped == value {
				Self { raw, escaped: None }
			} else {
				Self {
					raw,
					escaped: Some(escaped),
				}
			}
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
	use super::test_helper;
	use super::*;

	#[test]
	fn from_str() {
		let fixture = ObjectIdentity::from("hello world");
		fixture.assert_raw("hello world");
		fixture.assert_escaped("hello world");

		let fixture = ObjectIdentity::from(r#"hello\nworld"#);
		fixture.assert_raw(r#"hello\nworld"#);
		fixture.assert_escaped("hello\nworld");
	}
}
