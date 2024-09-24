use super::number::number;
use super::{array::array, boolean::boolean, null::null, object::object, string::string};
use crate::syntax_node::prelude::Node;
use combine as cmb;
use combine::parser;
use combine::parser::char as chr;
use combine::{Parser, Stream};
pub fn ws<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let space = cmb::satisfy::<I, _>(|c| match c {
		'\u{20}' => true,
		'\u{09}' => true,
		'\u{0A}' => true,
		'\u{0D}' => true,
		_ => false,
	});

	cmb::many::<String, I, _>(space)
}

fn value_<I: Stream<Token = char>>() -> impl Parser<I, Output = Node> {
	let v = boolean::<I>()
		.or(null())
		.or(string())
		.or(number())
		.or(array())
		.or(object());

	(ws(), v, ws()).map(|(l, v, t)| Node::new(v, l, t))
}
parser! {
	pub fn value[I]()(I)->Node
	where [I:Stream<Token=char>]{
		value_()
	}
}
#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::prelude::*;
	use crate::syntax_node::test_prelude::*;

	fn add_ws(s: &str) -> String {
		format!("{WS}{s}{WS}")
	}
	#[test]
	fn string() {
		let str = add_ws(r#""rust""#);
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_string("rust");
	}

	#[test]
	fn integer() {
		let str = add_ws("42");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_integer("42");
	}

	#[test]
	fn float() {
		let str = add_ws("42.195");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_float("42.195");

		let str = add_ws("42.1955e-1");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_float("42.1955e-1");
	}

	#[test]
	fn boolean() {
		let str = add_ws("true");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_true();

		let str = add_ws("false");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
	}

	#[test]
	fn null() {
		let str = add_ws("null");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		a.value().extract_terminal().assert_null();
	}

	#[test]
	fn empty_array() {
		let mut parser = super::value::<&str>();
		let (v, r) = parser.parse("[   ]").unwrap();
		assert_eq!(r, "");
		v.value().extract_array().value().assert_empty("   ")
	}
	#[test]
	fn array() {
		let str = add_ws("[1,2,3]");
		let mut parser = super::value::<&str>(); //::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		let arr = a.value().extract_array().value().extract_contents();

		assert_eq!(arr.len(), 3);
		arr[0].value().extract_terminal().assert_integer("1");
		arr[1].value().extract_terminal().assert_integer("2");
		arr[2].value().extract_terminal().assert_integer("3");
	}

	#[test]
	fn object() {
		let str = add_ws(r#"{"a": 1, "b": 2, "c": 3}"#);
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.assert_lead_trail(None, None);
		let obj = a.value().extract_object().value().extract_contents();

		assert_eq!(obj.len(), 3);
		obj[0].key().value().extract_terminal().assert_string("a");
		obj[0]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("1");

		obj[1].key().value().extract_terminal().assert_string("b");
		obj[1]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("2");

		obj[2].key().value().extract_terminal().assert_string("c");
		obj[2]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("3");
	}

	#[test]
	fn ws() {
		use crate::syntax_node::test_prelude::ws;

		for expected in WS.chars().map(|c| c.to_string()) {
			let mut parser = super::ws::<&str>();
			let (a, r) = parser.parse(&expected).unwrap();
			assert_eq!(r, "");
			assert_eq!(a, expected);
		}

		let mut parser = super::ws::<&str>();
		let (a, r) = parser.parse(WS).unwrap();
		assert_eq!(r, "");
		assert_eq!(a, ws());

		let (a, r) = parser.parse("").unwrap();
		assert_eq!("", a);
		assert_eq!("", r)
	}
}
