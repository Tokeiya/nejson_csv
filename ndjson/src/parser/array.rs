use super::value::{value, ws};
use crate::syntax_node::prelude::*;
use combine::{self as cmb, parser::char as chr, Parser, Stream};
use std::cell::RefCell;
use std::rc::Rc;

fn first<I: Stream<Token = char>>(
	logger: Rc<RefCell<Vec<String>>>,
) -> impl Parser<I, Output = Rc<Node>> {
	value::<I>(logger)
}

fn following<I: Stream<Token = char>>(
	logger: Rc<RefCell<Vec<String>>>,
) -> impl Parser<I, Output = Vec<Rc<Node>>> {
	let tmp = (chr::char(','), value::<I>(logger)).map(|(_, v)| v);
	cmb::many::<Vec<Rc<Node>>, I, _>(tmp)
}

fn contents<I: Stream<Token = char>>(
	logger: Rc<RefCell<Vec<String>>>,
) -> impl Parser<I, Output = NodeValue> {
	let empty = ws::<I>().map(|_| NodeValue::Array(NonTerminalNode::new(Vec::new())));

	let content = (first::<I>(logger.clone()), following(logger.clone())).map(|(a, b)| {
		let mut vec = b;
		vec.insert(0, a);

		for (idx, elem) in vec.iter().enumerate() {
			elem.set_identity(Identity::from(idx))
		}

		NodeValue::Array(NonTerminalNode::new(vec))
	});

	cmb::attempt(content).or(empty)
}

pub fn array<I: Stream<Token = char>>(
	logger: Rc<RefCell<Vec<String>>>,
) -> impl Parser<I, Output = NodeValue> {
	(chr::char('['), contents(logger), chr::char(']')).map(|(_, v, _)| v)
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

	fn gen_logger() -> Rc<RefCell<Vec<String>>> {
		Rc::new(RefCell::new(Vec::new()))
	}

	#[test]
	fn first() {
		let mut parser = super::first(gen_logger());
		let (a, r) = parser.parse("20").unwrap();
		assert_eq!(r, "");
		a.value().extract_terminal().assert_integer("20");
	}

	#[test]
	fn following() {
		let mut parser = super::following(gen_logger());
		let (a, r) = parser.parse("").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 0);

		let (a, r) = parser.parse(",1,2,3").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 3);
	}

	#[test]
	fn contents() {
		let mut parser =
			(super::first(gen_logger()), super::following(gen_logger())).map(|(a, b)| {
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

		let mut parser = super::contents::<&str>(gen_logger());

		let (a, r) = parser.parse("1").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.extract_array().value().len(), 1);

		let (a, r) = parser.parse("1,2").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.extract_array().value().len(), 2);

		let (a, r) = parser.parse("   ").unwrap();
		assert_eq!(r, "");
		assert_eq!(a.extract_array().value().len(), 0);
	}

	#[test]
	fn array() {
		let str = generate_array(vec![r#""rust""#, "42", "null", "true", "false", "42.195"]);
		let mut parser = super::array::<&str>(gen_logger());

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");

		let act = act.extract_array().value();
		assert_eq!(act.len(), 6);

		let piv = &act[0];
		piv.value().extract_terminal().assert_string("rust");
		piv.identity().assert_index(0);

		let piv = &act[1];
		piv.value().extract_terminal().assert_integer("42");
		piv.identity().assert_index(1);

		let piv = &act[2];
		piv.value().extract_terminal().assert_null();
		piv.identity().assert_index(2);

		let piv = &act[3];
		piv.value().extract_terminal().assert_true();
		piv.identity().assert_index(3);

		let piv = &act[4];
		piv.value().extract_terminal().assert_false();
		piv.identity().assert_index(4);

		let piv = &act[5];
		piv.value().extract_terminal().assert_float("42.195");
		piv.identity().assert_index(5);
	}

	#[test]
	fn empty_array() {
		let str = "[]";
		let mut parser = super::array::<&str>(gen_logger());

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.extract_array().value().len(), 0);

		let str = format!("[{WS}]");
		let mut parser = super::array::<&str>(gen_logger());
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.extract_array().value().len(), 0);
	}

	#[test]
	fn err() {
		let mut parser = super::array::<&str>(gen_logger());
		assert!(parser.parse("[1,2,3,]").is_err());
	}
}
