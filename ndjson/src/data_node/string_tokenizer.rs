use crate::data_node::string_parse_error::StringParseError;
use std::collections::HashSet;
use std::iter::{Iterator, Peekable};
use std::str::CharIndices;
use std::sync::LazyLock;
type Iter<'a> = Peekable<CharIndices<'a>>;
pub struct StringTokenizer<'a>(&'a str);

static ESCAPE: LazyLock<HashSet<char>> = LazyLock::new(|| {
	let mut set = HashSet::new();
	set.insert('"');
	set.insert('\\');
	set.insert('/');
	set.insert('b');
	set.insert('f');
	set.insert('n');
	set.insert('r');
	set.insert('t');

	set
});
impl StringTokenizer<'_> {
	pub fn new(scr: &str) -> StringTokenizer<'_> {
		StringTokenizer(scr)
	}

	fn surrogate_pair<'a>(&'a mut self) -> Option<Result<&'a str, StringParseError>> {
		dbg!(self.0.len());
		dbg!(self.0);

		if self.0.len() < 12 {
			self.0 = "";
			return Some(Err(StringParseError::UnexpectedEof()));
		};

		if let Ok(code) = u16::from_str_radix(&self.0[2..6], 16) {
			if code >= 0xD800 && code <= 0xDBFF {
				if let Ok(code) = u16::from_str_radix(&self.0[8..12], 16) {
					if code >= 0xDC00 && code <= 0xDFFF {
						let ret = Some(Ok(&self.0[0..12]));
						self.0 = &self.0[12..];
						ret
					} else {
						let ret = Some(Err(StringParseError::InvalidSurrogate {
							first: self.0[..6].to_string(),
							second: self.0[6..12].to_string(),
						}));

						self.0 = &self.0[12..];
						return ret;
					}
				} else {
					let ret = Some(Err(StringParseError::InvalidSurrogate {
						first: self.0[..6].to_string(),
						second: self.0[6..12].to_string(),
					}));

					self.0 = &self.0[12..];
					return ret;
				}
			} else {
				let ret = Some(Err(StringParseError::InvalidSurrogate {
					first: self.0[..6].to_string(),
					second: "".to_string(),
				}));

				self.0 = &self.0[6..];
				return ret;
			}
		} else {
			let ret = Some(Err(StringParseError::InvalidSurrogate {
				first: self.0[..6].to_string(),
				second: self.0[6..12].to_string(),
			}));

			self.0 = &self.0[12..];
			return ret;
		}
	}

	fn unicode<'a>(&'a mut self, mut iter: Iter<'a>) -> Option<Result<&'a str, StringParseError>> {
		let (_, c) = iter.next().unwrap();

		if c != 'u' {
			unreachable!()
		};

		if self.0.len() < 6 {
			self.0 = "";
			Some(Err(StringParseError::UnexpectedEof()))
		} else {
			if let Ok(code) = u16::from_str_radix(&self.0[2..6], 16) {
				if code >= 0xD800u16 && code <= 0xDFFF {
					self.surrogate_pair()
				} else {
					let ret = Some(Ok(&self.0[..6]));
					self.0 = &self.0[6..];
					ret
				}
			} else {
				self.0 = &self.0[6..];

				let ret = Some(Err(StringParseError::InvalidEscape(
					self.0[..6].to_string(),
				)));

				ret
			}
		}
	}

	fn escape<'a>(&'a mut self, mut iter: Iter<'a>) -> Option<Result<&'a str, StringParseError>> {
		// unescaped = %x20-21 / %x23-5B / %x5D-10FFFF

		let (_, c) = iter.next().unwrap();

		if c != '\\' {
			unreachable!()
		};

		if let Some((i, c)) = iter.peek() {
			if ESCAPE.contains(c) {
				let ret = Some(Ok(&self.0[..=*i]));
				self.0 = &self.0[i + 1..];
				ret
			} else if c == &'u' {
				self.unicode(iter)
			} else {
				let str = &self.0[..=*i];
				let str = str.to_string();

				let ret = Some(Err(StringParseError::InvalidEscape(str)));
				self.0 = &self.0[i + 1..];
				ret
			}
		} else {
			let ret = Some(Err(StringParseError::UnexpectedEof()));
			self.0 = "";
			ret
		}
	}
	#[inline]
	fn is_unescaped(c: char) -> bool {
		todo!()
	}

	pub fn next(&mut self) -> Option<Result<&str, StringParseError>> {
		let mut chars = self.0.char_indices().peekable();

		let (i, c) = chars.peek()?;

		if *c == '\\' {
			self.escape(chars)
		} else {
			let ret = Some(Ok(&self.0[*i..=*i]));
			self.0 = &self.0[i + 1..];
			ret
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
	fn next_normal() {
		let mut fixture = StringTokenizer::new("hello");
		assert_eq!(fixture.next().unwrap().unwrap(), "h");
		assert_eq!(fixture.next().unwrap().unwrap(), "e");
		assert_eq!(fixture.next().unwrap().unwrap(), "l");
		assert_eq!(fixture.next().unwrap().unwrap(), "l");
		assert_eq!(fixture.next().unwrap().unwrap(), "o");
		assert!(fixture.next().is_none());
	}

	#[test]
	fn next_escape() {
		let mut fixture = StringTokenizer::new(r#"\"\\\/\b\f\n\r\t"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\""#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\\"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\/"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\b"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\f"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\n"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\r"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\t"#);
		assert!(fixture.next().is_none());
	}

	#[test]
	fn next_unicode() {
		let mut fixture = StringTokenizer::new(r#"\u0068\u0065\u006C\u006C\u006F"#);

		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u0068"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u0065"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u006C"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u006C"#);
		assert_eq!(fixture.next().unwrap().unwrap(), r#"\u006F"#);
		assert!(fixture.next().is_none());
	}

	#[test]
	fn surrogate_pair_next() {
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
		let mut fixture = StringTokenizer::new(r#"\"#);
		fixture.next().unwrap().unwrap_err().assert_unexpected_eof();

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
