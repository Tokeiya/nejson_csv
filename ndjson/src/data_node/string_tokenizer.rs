use crate::data_node::string_parse_error::StringParseError;
use std::str::Chars;

pub struct StringTokenizer<'a>(&'a str);

impl StringTokenizer<'_> {
	pub fn new(scr: &str) -> StringTokenizer<'_> {
		StringTokenizer(scr)
	}

	fn cut(&mut self, idx: usize) -> Option<Result<&'_ str, StringParseError>> {
		let ret = &self.0[..idx];
		self.0 = &self.0[idx..];
		Some(Ok(ret))
	}
	
	pub fn unicode(&mut self,mut chars:impl Iterator<Item=(usize,char)>)->Option<Result<&'_ str,StringParseError>>{
		let mut cnt=0usize;

		loop {
			let piv=chars.next();
			
			if let None=piv{
				let tmp=
			}
			
		}
	}

	pub fn next(&mut self) -> Option<Result<&str, StringParseError>> {
		let mut chrs = self.0.chars().enumerate();

		if let Some((i, c)) = chrs.next() {
			if c == '\\' {
				if let Some((i, c)) = chrs.next() {
					match c {
						'"' => self.cut(i),
						'\\' => self.cut(i),
						'/' => self.cut(i),
						'b' => self.cut(i),
						'f' => self.cut(i),
						'n' => self.cut(i),
						'r' => self.cut(i),
						't' => self.cut(i),

						_ => Some(Err(StringParseError::InvalidEscape(format!("\\{c}")))),
					}
				} else {
					Some(Err(StringParseError::InvalidEscape("\\".to_string())))
				}
			} else {
				let ret = &self.0[..1];
				self.0 = &self.0[1..];
				Some(Ok(ret))
			}
		} else {
			None
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn new() {
		let fixture = StringTokenizer::new("hello");
		assert_eq!(fixture.0, "hello");
	}

	#[test]
	fn next() {
		let mut fixture = StringTokenizer::new("");
		assert!(fixture.next().is_none());

		let mut fixture = StringTokenizer::new("hello");
		assert_eq!(fixture.next().unwrap().unwrap(), "h");
		assert_eq!(fixture.next().unwrap().unwrap(), "e");
		assert_eq!(fixture.next().unwrap().unwrap(), "l");
		assert_eq!(fixture.next().unwrap().unwrap(), "l");
		assert_eq!(fixture.next().unwrap().unwrap(), "o");
		assert!(fixture.next().is_none());

		let mut fixture = StringTokenizer::new(r#"\"\\\/\b\f\n\r\t\u0061"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\\"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\/"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\b"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\f"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\n"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\r"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\t"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u0061"#);
		assert!(fixture.next().is_none());

		let mut fixture =
			StringTokenizer::new(r#"\uD83D\uDE0A\uD83E\uDEE0\uD83D\uDE23\uD83D\uDE18"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\uD83D\uDE0A"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\uD83E\uDEE0"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\uD83D\uDE23"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\uD83D\uDE18"#);
		assert!(fixture.next().is_none());
	}

	#[test]
	fn invalid_next() {
		let mut fixture = StringTokenizer::new(r#"ab\cd"#);
		assert_eq!(fixture.next().unwrap().unwrap(), "a");
		assert_eq!(fixture.next().unwrap().unwrap(), "b");

		fixture
			.next()
			.unwrap()
			.unwrap_err()
			.assert_invalid_escape(r#"\c"#);

		assert_eq!(fixture.next().unwrap().unwrap(), "d");

		assert!(fixture.next().is_none());

		let mut fixture = StringTokenizer::new(r#"ab\uDE0A\uD83E\uDEE0"#);
		assert_eq!(fixture.next().unwrap().unwrap(), "a");
		assert_eq!(fixture.next().unwrap().unwrap(), "b");

		fixture
			.next()
			.unwrap()
			.unwrap_err()
			.assert_invalid_surrogate(r#"\uDE0A"#, "");
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\uD83E\uDEE0"#);
	}
}
