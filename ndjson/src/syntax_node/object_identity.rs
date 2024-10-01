use crate::data_node::{StringParseError, StringTokenizer};

pub struct ObjectIdentity {
	raw: String,
	escaped: Option<String>,
}

impl From<&str> for ObjectIdentity {
	fn from(value: &str) -> Self {
		let raw = value.to_string();
		let mut tokenizer = StringTokenizer::new(&raw);
	}
}

impl ObjectIdentity {
	pub fn raw(&self) -> &str {
		todo!()
	}

	pub fn escaped(&self) -> &str {
		todo!()
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
			assert_eq!(&self.escaped, expected);
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
