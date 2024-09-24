use super::value::{value, ws};
use crate::syntax_node::prelude;
use crate::syntax_node::prelude::*;
use combine::{
	self as cmb,
	parser::{self as psr, char as chr},
	Parser, Stream,
};

fn first<I: Stream<Token = char>>() -> impl Parser<I, Output = Node> {
	value::<I>()
}

fn following<I: Stream<Token = char>>() -> impl Parser<I, Output = Vec<Node>> {
	let tmp = (chr::char(','), value::<I>()).map(|(_, v)| v);
	cmb::many::<Vec<Node>, I, _>(tmp)
}

fn contents<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	let empty = ws::<I>().map(|s| NodeValue::Array(ArrayNode::empty(s)));

	let content = (first::<I>(), following()).map(|(a, b)| {
		let mut v = b;
		v.insert(0, a);
		NodeValue::Array(ArrayNode::new(v))
	});

	cmb::attempt(content).or(empty)
}

pub fn array<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	(chr::char('['), contents(), chr::char(']')).map(|(_, v, _)| v)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::test_prelude::*;

	fn generate_array(scr: Vec<&str>) -> String {
		let mut buff = String::from('[');

		for elem in scr {
			buff.push_str(WS);
			buff.push_str(elem);
			buff.push_str(WS);
			buff.push(',');
		}

		buff.remove(buff.len() - 1);
		buff.push(']');

		buff
	}

	#[test]
	fn first() {
		let mut parser = super::first();
		let (a, r) = parser.parse("20").unwrap();
		assert_eq!(r, "");
		a.value().extract_terminal().assert_integer("20");
	}

	#[test]
	fn following() {
		let mut parser = super::following();
		let (a, r) = parser.parse("").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 0);

		let (a, r) = parser.parse(",1,2,3").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 3);
	}

	#[test]
	fn contents() {
		let mut parser = (super::first(), super::following()).map(|(a, b)| {
			let mut v = b;
			v.insert(0, a);
			v
		});

		let (a, r) = parser.parse("1").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 1);

		let (a, r) = parser.parse("1,2").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 2);

		let mut parser = super::contents::<&str>();

		let (a, r) = parser.parse("1").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.extract_array().value().extract_contents().len(), 1);

		let (a, r) = parser.parse("1,2").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.extract_array().value().extract_contents().len(), 2);

		let (a, r) = parser.parse("   ").unwrap();
		assert_eq!(r, "");
		a.extract_array().value().assert_empty("   ");
	}

	#[test]
	fn array() {
		let str = generate_array(vec![r#""rust""#, "42", "null", "true", "false", "42.195"]);
		let mut parser = super::array::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");

		let act = act.extract_array().value().extract_contents();
		assert_eq!(act.len(), 6);

		let piv = &act[0];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_string("rust");

		let piv = &act[1];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_integer("42");

		let piv = &act[2];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_null();

		let piv = &act[3];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_true();

		let piv = &act[4];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_false();

		let piv = &act[5];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_float("42.195");
	}

	#[test]
	fn empty_array() {
		let str = "[]";
		let mut parser = super::array::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.extract_array().value().assert_empty("");

		let str = format!("[{WS}]");
		let mut parser = super::array::<&str>();
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.extract_array().value().assert_empty(WS);
	}

	#[test]
	fn err() {
		let mut parser = super::array::<&str>();
		assert!(parser.parse("[1,2,3,]").is_err());
	}
}
