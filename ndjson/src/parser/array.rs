use super::value::{value, ws};
use crate::syntax_node::prelude::*;
use combine::{self as cmb, parser::char as chr, Parser, Stream};
use std::rc::Rc;

fn first<I: Stream<Token = char>>() -> impl Parser<I, Output = Rc<Node>> {
	value::<I>()
}

fn following<I: Stream<Token = char>>() -> impl Parser<I, Output = Vec<Rc<Node>>> {
	let tmp = (chr::char(','), value::<I>()).map(|(_, v)| v);
	cmb::many::<Vec<Rc<Node>>, I, _>(tmp)
}

fn contents<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	let empty = ws::<I>().map(|s| NodeValue::Array(ArrayNode::empty(s)));

	let content = (first::<I>(), following()).map(|(a, b)| {
		let mut v = Vec::new();
		v.push(ArrayElement::new(0, a));

		for (i, n) in b.into_iter().enumerate() {
			v.push(ArrayElement::new(i + 1, n));
		}
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
		piv.value().assert_lead_trail(None, None);
		piv.value().value().extract_terminal().assert_string("rust");
		piv.assert_index(0);

		let piv = &act[1];
		piv.value().assert_lead_trail(None, None);
		piv.value().value().extract_terminal().assert_integer("42");
		piv.assert_index(1);

		let piv = &act[2];
		piv.value().assert_lead_trail(None, None);
		piv.value().value().extract_terminal().assert_null();
		piv.assert_index(2);

		let piv = &act[3];
		piv.value().assert_lead_trail(None, None);
		piv.value().value().extract_terminal().assert_true();
		piv.assert_index(3);

		let piv = &act[4];
		piv.value().assert_lead_trail(None, None);
		piv.value().value().extract_terminal().assert_false();
		piv.assert_index(4);

		let piv = &act[5];
		piv.value().assert_lead_trail(None, None);
		piv.value()
			.value()
			.extract_terminal()
			.assert_float("42.195");
		piv.assert_index(5);
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
