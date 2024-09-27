use crate::data_node::string_parse_error::StringParseError;
use std::collections::HashSet;
use std::iter::{IntoIterator, Iterator, Peekable};
use std::str::{CharIndices, Chars};
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

	fn surrogate_pair<'a>(
		mut iter: Iter<'a>,
	) -> (Iter<'a>, Option<Result<&'a str, StringParseError>>) {
		todo!()
	}

	fn unicode<'a>(mut iter: Iter<'a>) -> (Iter<'a>, Option<Result<&'a str, StringParseError>>) {
		todo!()
	}

	fn escape<'a>(mut iter: Iter<'a>) -> (Iter<'a>, Option<Result<&'a str, StringParseError>>) {
		let memo = iter.clone();

		if let Some((idx, chr)) = iter.next() {
			if chr != '\\' {
				unreachable!()
			};

			if let Some((idx, chr)) = iter.peek() {
				if ESCAPE.contains(&chr) {}
			}
		} else {
			unreachable!()
		}

		todo!()
	}

	pub fn next(&mut self) -> Option<Result<&str, StringParseError>> {
		let mut chrs = self.0.char_indices().peekable();

		let (idx, chr) = chrs.peek()?;
		dbg!(self.0);

		if chr == &'\\' {
			let (mut iter, opt) = Self::escape(chrs);

			if let Some((i, _)) = iter.peek() {
				self.0 = &self.0[*i..];
			} else {
				self.0 = "";
			}

			return opt;
		} else {
			let ret = &self.0[*idx..=*idx];
			self.0 = &self.0[idx + 1..];
			Some(Ok(ret))
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
