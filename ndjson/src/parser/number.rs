use crate::syntax_node::prelude::*;
use combine as cmb;
use combine::parser::char as chr;
use combine::{Parser, Stream};

use combine::parser::combinator;
fn integer<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let minus = cmb::optional::<I, _>(chr::char::<I>('-')).map(|c| match c {
		None => "".to_string(),
		Some(_) => '-'.to_string(),
	});

	let zero = chr::char::<I>('0')
		.skip(combinator::not_followed_by(chr::digit()))
		.map(|_| "0".to_string());

	let one_nine = cmb::satisfy::<I, _>(|c| c >= '1' && c <= '9');

	let tmp = (one_nine, cmb::many::<String, I, _>(chr::digit::<I>())).map(|(f, s)| {
		let mut buff = String::from(f);
		buff.push_str(s.as_str());
		buff
	});

	let tmp = zero.or(tmp);

	(minus, tmp).map(|(m, t)| {
		let mut buff = m;
		buff.push_str(t.as_str());
		buff
	})
}

fn fraction<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	(chr::char('.'), cmb::many1::<String, I, _>(chr::digit())).map(|(_, d)| format!(".{d}"))
}

fn exponent<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let e = chr::char::<I>('e').or(chr::char('E'));

	let sign = chr::char('+').or(chr::char('-'));
	let sign = cmb::optional(sign);

	let digits = cmb::many1::<String, I, _>(chr::digit());

	(e, sign, digits).map(|(e, s, d)| {
		let mut buff = String::from(e);

		match s {
			None => {}
			Some(s) => buff.push(s),
		};

		buff.push_str(&d);
		buff
	})
}

pub fn number<I: Stream<Token = char>>() -> impl Parser<I, Output = TerminalNode> {
	(
		integer::<I>(),
		cmb::optional::<I, _>(fraction()),
		cmb::optional::<I, _>(exponent()),
	)
		.map(|(i, f, e)| {
			let mut is_int = true;
			let mut buff = i;

			match f {
				None => {}
				Some(x) => {
					buff.push_str(x.as_str());
					is_int = false;
				}
			};

			match e {
				None => {}
				Some(x) => {
					buff.push_str(x.as_str());
					is_int = false;
				}
			}

			if is_int {
				TerminalNode::new(TerminalNodeType::Integer, buff)
			} else {
				TerminalNode::new(TerminalNodeType::Float, buff)
			}
		})
}

#[cfg(test)]
mod test {
	use super::*;

	fn assert(input: &(String, &str), expected: &str, rem: Option<&str>) {
		assert_eq!(input.0.as_str(), expected);

		let rem = if let Some(s) = rem { s } else { "" };
		assert_eq!(input.1, rem);
	}

	#[test]
	fn integer() {
		for expected in (-10..10).map(|x| x.to_string()) {
			let mut parser = super::integer::<&str>();
			let act = parser.parse(&expected).unwrap();

			assert(&act, &expected, None)
		}

		let mut parser = super::integer::<&str>();
		assert(&parser.parse("-0").unwrap(), "-0", None);
		assert!(parser.parse("+0").is_err());
		assert!(parser.parse("01").is_err());
	}

	#[test]
	fn fraction() {
		for elem in (0..=100).map(|x| format!(".{x}")) {
			let mut parser = super::fraction::<&str>();
			assert(&parser.parse(&elem).unwrap(), &elem, None);
		}

		let mut parser = super::fraction::<&str>();
		assert!(parser.parse(".").is_err())
	}

	#[test]
	fn exponent() {
		for vec in (0..100).map(|x| {
			vec![
				format!("e{x}"),
				format!("E{x}"),
				format!("e+{x}"),
				format!("E+{x}"),
				format!("e-{x}"),
				format!("E-{x}"),
			]
		}) {
			for elem in vec {
				let mut parser = super::exponent::<&str>();
				assert(&parser.parse(&elem).unwrap(), &elem, None);
			}
		}

		let mut parser = super::exponent::<&str>();
		assert!(&parser.parse("e+").is_err());
		assert!(&parser.parse("E+").is_err());
	}

	#[test]
	fn number() {
		fn assert(
			(act, rem): (TerminalNode, &str),
			expected: &str,
			expected_type: TerminalNodeType,
		) {
			act.assert(expected_type, expected);
			assert_eq!(rem, "");
		}

		let mut parser = super::number::<&str>();

		assert(parser.parse("0").unwrap(), "0", TerminalNodeType::Integer);
		assert(parser.parse("-0").unwrap(), "-0", TerminalNodeType::Integer);

		assert(parser.parse("42").unwrap(), "42", TerminalNodeType::Integer);
		assert(
			parser.parse("-42").unwrap(),
			"-42",
			TerminalNodeType::Integer,
		);

		assert(
			parser.parse("42.195").unwrap(),
			"42.195",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195").unwrap(),
			"-42.195",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42e3").unwrap(),
			"42e3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42e3").unwrap(),
			"-42e3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42E3").unwrap(),
			"42E3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42E3").unwrap(),
			"-42E3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42e+3").unwrap(),
			"42e+3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42e+3").unwrap(),
			"-42e+3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42E+3").unwrap(),
			"42E+3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42E+3").unwrap(),
			"-42E+3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42e-3").unwrap(),
			"42e-3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42e-3").unwrap(),
			"-42e-3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42.195E-3").unwrap(),
			"42.195E-3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195E-3").unwrap(),
			"-42.195E-3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42.195e-3").unwrap(),
			"42.195e-3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195e-3").unwrap(),
			"-42.195e-3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42.195E+3").unwrap(),
			"42.195E+3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195E+3").unwrap(),
			"-42.195E+3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42.195e+3").unwrap(),
			"42.195e+3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195e+3").unwrap(),
			"-42.195e+3",
			TerminalNodeType::Float,
		);

		assert(
			parser.parse("42.195e3").unwrap(),
			"42.195e3",
			TerminalNodeType::Float,
		);
		assert(
			parser.parse("-42.195e3").unwrap(),
			"-42.195e3",
			TerminalNodeType::Float,
		);

		assert!(parser.parse("-42.").is_err());
		assert!(parser.parse("-42.e3").is_err());
	}
}
