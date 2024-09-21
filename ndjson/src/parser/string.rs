use super::super::syntax_node::prelude::*;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

enum O {
	Char(char),
	String(String),
}

fn unescaped<I: Stream<Token = char>>() -> impl Parser<I, Output = O> {
	cmb::satisfy::<I, _>(|c| {
		if c >= '\u{20}' && c <= '\u{21}' {
			true
		} else if c >= '\u{23}' && c <= '\u{5B}' {
			true
		} else if c >= '\u{5D}' && c <= '\u{10FFFF}' {
			true
		} else {
			false
		}
	})
	.map(|c| O::Char(c))
}

fn escaped<I: Stream<Token = char>>() -> impl Parser<I, Output = O> {
	let digits = cmb::parser::repeat::count_min_max::<String, I, _>(4, 4, chr::hex_digit());
	let code_point = (chr::char::<I>('u'), digits).map(|(_, d)| O::String(format!("\\u{d}")));

	let escape_identity = cmb::satisfy::<I, _>(|c| match c {
		'"' => true,
		'\\' => true,
		'/' => true,
		'b' => true,
		'f' => true,
		'n' => true,
		'r' => true,
		't' => true,
		_ => false,
	})
	.map(|c| O::String(format!("\\{c}")));

	(chr::char('\\'), code_point.or(escape_identity)).map(|(_, o)| o)
}

//Quote is captured by the caller
pub fn string<I: Stream<Token = char>>() -> impl Parser<I, Output = (String, TerminalNodeType)> {
	(
		chr::char('"'),
		cmb::many::<Vec<O>, I, _>(unescaped().or(escaped())),
		chr::char('"'),
	)
		.map(|(_, c, _)| {
			let mut buff = String::from('"');

			for elem in c {
				match elem {
					O::Char(c) => buff.push(c),
					O::String(s) => buff.push_str(&s),
				}
			}

			buff.push('"');

			(buff, TerminalNodeType::String)
		})
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn unescaped() {
		let mut parser = super::unescaped::<&str>();

		assert!(parser.parse("\u{19}").is_err());

		let (c, r) = parser.parse("\u{20}").unwrap();
		assert!(matches!(c, O::Char(c) if c == '\u{20}'));
		assert_eq!(r, "");

		let (c, r) = parser.parse("\u{21}").unwrap();
		assert!(matches!(c, O::Char(c) if c == '\u{21}'));
		assert_eq!(r, "");

		assert!(parser.parse("\u{22}").is_err());

		let (o, r) = parser.parse("\u{23}").unwrap();
		assert!(matches!(o, O::Char(c) if c == '\u{23}'));
		assert_eq!(r, "");

		let (o, r) = parser.parse("\u{5B}").unwrap();
		assert!(matches!(o, O::Char(c) if c == '\u{5B}'));
		assert_eq!(r, "");

		assert!(parser.parse("\u{5C}").is_err());

		let (o, r) = parser.parse("\u{5D}").unwrap();
		assert!(matches!(o, O::Char(c) if c == '\u{5D}'));
		assert_eq!(r, "");

		let (o, r) = parser.parse("\u{10FFFF}").unwrap();
		assert!(matches!(o, O::Char(c) if c == '\u{10FFFF}'));
		assert_eq!(r, "");
	}

	#[test]
	fn escaped() {
		let mut parser = super::escaped::<&str>();

		let (o, r) = parser.parse(r#"\""#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\""#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\\"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\\"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\/"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\/"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\b"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\b"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\f"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\f"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\n"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\n"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\r"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\r"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\t"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\t"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\u0041"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\u0041"#));
		assert_eq!(r, "");

		let (o, r) = parser.parse(r#"\u0061"#).unwrap();
		assert!(matches!(o, O::String(s) if s == r#"\u0061"#));
		assert_eq!(r, "");
	}

	#[test]
	fn string() {
		let mut parser = super::string::<&str>();

		let ((s, t), r) = parser.parse(r#""foo""#).unwrap();

		assert_eq!(
			parser.parse(r#""foo""#).unwrap(),
			((r#""foo""#.to_string(), TerminalNodeType::String), "")
		);

		assert_eq!(
			parser.parse(r#""\u0041\u0061""#).unwrap(),
			(
				(r#""\u0041\u0061""#.to_string(), TerminalNodeType::String),
				""
			)
		);

		assert_eq!(
			parser.parse(r#""\"\\\/\b\f\n\r\t\u0061""#).unwrap(),
			(
				(
					r#""\"\\\/\b\f\n\r\t\u0061""#.to_string(),
					TerminalNodeType::String
				),
				""
			)
		);

		assert_eq!(
			parser.parse(r#""hello world""#).unwrap(),
			(
				(r#""hello world""#.to_string(), TerminalNodeType::String),
				""
			)
		);
	}
}
