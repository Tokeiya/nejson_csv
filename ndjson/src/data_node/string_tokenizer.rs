use super::string_token::StringToken;
use crate::data_node::string_parse_error::StringParseError;
use std::collections::HashMap;
use std::iter::{Iterator, Peekable};
use std::str::CharIndices;
use std::sync::LazyLock;

type Iter<'a> = Peekable<CharIndices<'a>>;
pub struct StringTokenizer<'a>(&'a str);

static ESCAPE: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
	let mut set = HashMap::new();
	set.insert('"', '"');
	set.insert('\\', '\\');
	set.insert('/', '/');
	set.insert('b', '\u{0008}');
	set.insert('f', '\u{000C}');
	set.insert('n', '\u{000A}');
	set.insert('r', '\u{000D}');
	set.insert('t', '\u{0009}');

	set
});
impl StringTokenizer<'_> {
	pub fn new(scr: &str) -> StringTokenizer<'_> {
		StringTokenizer(scr)
	}

	fn surrogate_pair(&mut self) -> Option<Result<StringToken<'_>, StringParseError>> {
		if self.0.len() < 12 {
			self.0 = "";
			return Some(Err(StringParseError::UnexpectedEof()));
		};

		if let Ok(hsg) = u32::from_str_radix(&self.0[2..6], 16) {
			if hsg >= 0xD800 && hsg <= 0xDBFF {
				if let Ok(lsg) = u32::from_str_radix(&self.0[8..12], 16) {
					if lsg >= 0xDC00 && lsg <= 0xDFFF {
						self.0 = &self.0[12..];

						let tmp = 0x01_00_00u32 + (hsg - 0xD8_00) * 0x04_00 + (lsg - 0xDC_00);
						let tmp = char::from_u32(tmp).unwrap();
						Some(Ok(StringToken::Char(tmp)))
					} else {
						let ret = Some(Err(StringParseError::InvalidSurrogate {
							first: self.0[..6].to_string(),
							second: self.0[6..12].to_string(),
						}));

						self.0 = &self.0[12..];
						ret
					}
				} else {
					let ret = Some(Err(StringParseError::InvalidSurrogate {
						first: self.0[..6].to_string(),
						second: self.0[6..12].to_string(),
					}));

					self.0 = &self.0[12..];
					ret
				}
			} else {
				let ret = Some(Err(StringParseError::InvalidSurrogate {
					first: self.0[..6].to_string(),
					second: "".to_string(),
				}));

				self.0 = &self.0[6..];
				ret
			}
		} else {
			let ret = Some(Err(StringParseError::InvalidSurrogate {
				first: self.0[..6].to_string(),
				second: self.0[6..12].to_string(),
			}));

			self.0 = &self.0[12..];
			ret
		}
	}

	fn unicode<'a>(
		&'a mut self,
		mut iter: Iter<'a>,
	) -> Option<Result<StringToken<'a>, StringParseError>> {
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
					self.0 = &self.0[6..];
					let tmp = char::from_u32(code as u32).unwrap();
					Some(Ok(StringToken::Char(tmp)))
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

	fn escape<'a>(
		&'a mut self,
		mut iter: Iter<'a>,
	) -> Option<Result<StringToken<'a>, StringParseError>> {
		// unescaped = %x20-21 / %x23-5B / %x5D-10FFFF

		let (_, c) = iter.next().unwrap();

		if c != '\\' {
			unreachable!()
		};

		if let Some((i, c)) = iter.peek() {
			if let Some(c) = ESCAPE.get(&c) {
				self.0 = &self.0[i + 1..];
				Some(Ok(StringToken::Char(*c)))
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
	fn is_unescaped(c: &char) -> bool {
		(c >= &'\u{20}' && c <= &'\u{21}')
			|| (c >= &'\u{23}' && c <= &'\u{5B}')
			|| (c >= &'\u{5D}' && c <= &'\u{10FFFF}')
	}

	pub fn next(&mut self) -> Option<Result<StringToken, StringParseError>> {
		let mut chars = self.0.char_indices().peekable();

		let (_, c) = chars.peek()?;

		if c == &'\\' {
			self.escape(chars)
		} else {
			for (i, c) in chars {
				if c == '\\' {
					let str = &self.0[..i];
					self.0 = &self.0[i..];
					return Some(Ok(StringToken::String(str)));
				}

				if !Self::is_unescaped(&c) {
					self.0 = &self.0[i..];
					return Some(Err(StringParseError::invalid_unescaped(c)));
				}
			}

			let str = self.0;
			self.0 = "";
			return Some(Ok(StringToken::String(str)));
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
	fn is_unescaped() {
		assert!(!StringTokenizer::is_unescaped(&'\u{19}'));
		assert!(StringTokenizer::is_unescaped(&'\u{20}'));
		assert!(StringTokenizer::is_unescaped(&'\u{21}'));
		assert!(!StringTokenizer::is_unescaped(&'\u{22}'));
		assert!(StringTokenizer::is_unescaped(&'\u{23}'));
		assert!(StringTokenizer::is_unescaped(&'\u{5B}'));
		assert!(!StringTokenizer::is_unescaped(&'\u{5C}'));
		assert!(StringTokenizer::is_unescaped(&'\u{5D}'));
		assert!(StringTokenizer::is_unescaped(&'\u{10FFFF}'));
	}

	#[test]
	fn next_normal() {
		let mut fixture = StringTokenizer::new("hello");
		fixture.next().unwrap().unwrap().assert_string("hello");
		assert!(fixture.next().is_none());
	}

	#[test]
	fn complex() {
		let mut fixture = StringTokenizer::new(r#"hello\tworld"#);
		fixture.next().unwrap().unwrap().assert_string("hello");
		fixture.next().unwrap().unwrap().assert_char('\t');
		fixture.next().unwrap().unwrap().assert_string("world");
		assert!(fixture.next().is_none());
	}

	#[test]
	fn next_escape() {
		let mut fixture = StringTokenizer::new(r#"\"\\\/\b\f\n\r\t"#);
		fixture.next().unwrap().unwrap().assert_char('\"');
		fixture.next().unwrap().unwrap().assert_char('\\');
		fixture.next().unwrap().unwrap().assert_char('/');
		fixture.next().unwrap().unwrap().assert_char('\u{0008}');
		fixture.next().unwrap().unwrap().assert_char('\u{000C}');
		fixture.next().unwrap().unwrap().assert_char('\n');
		fixture.next().unwrap().unwrap().assert_char('\r');
		fixture.next().unwrap().unwrap().assert_char('\t');
		assert!(fixture.next().is_none());
	}

	#[test]
	fn next_unicode() {
		let mut fixture = StringTokenizer::new(r#"\u0068\u0065\u006C\u006C\u006F"#);

		fixture.next().unwrap().unwrap().assert_char('h');
		fixture.next().unwrap().unwrap().assert_char('e');
		fixture.next().unwrap().unwrap().assert_char('l');
		fixture.next().unwrap().unwrap().assert_char('l');
		fixture.next().unwrap().unwrap().assert_char('o');
		assert!(fixture.next().is_none());
	}

	#[test]
	fn surrogate_pair_next() {
		let mut fixture =
			StringTokenizer::new(r#"\uD83D\uDE0A\uD83E\uDEE0\uD83D\uDE23\uD83D\uDE18"#);

		fixture.next().unwrap().unwrap().assert_char('\u{1F60A}');
		fixture.next().unwrap().unwrap().assert_char('🫠');
		fixture.next().unwrap().unwrap().assert_char('\u{1F623}');
		fixture.next().unwrap().unwrap().assert_char('\u{1F618}');
		assert!(fixture.next().is_none());
	}

	#[test]
	fn invalid_next() {
		let mut fixture = StringTokenizer::new(r#"\"#);
		fixture.next().unwrap().unwrap_err().assert_unexpected_eof();

		let mut fixture = StringTokenizer::new(r#"ab\cd"#);
		fixture.next().unwrap().unwrap().assert_string("ab");

		fixture
			.next()
			.unwrap()
			.unwrap_err()
			.assert_invalid_escape(r#"\c"#);

		fixture.next().unwrap().unwrap().assert_string("d");
		assert!(fixture.next().is_none());

		let mut fixture = StringTokenizer::new(r#"ab\uDE0A\uD83E\uDEE0"#);
		fixture.next().unwrap().unwrap().assert_string("ab");

		fixture
			.next()
			.unwrap()
			.unwrap_err()
			.assert_invalid_surrogate(r#"\uDE0A"#, "");

		fixture.next().unwrap().unwrap().assert_char('🫠');
		assert!(fixture.next().is_none());
	}
}
