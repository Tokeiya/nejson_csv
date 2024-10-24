use super::string::string as string_parser;
use super::value::{value, ws};
use crate::log::prelude::Logger;
use crate::syntax_node::prelude::*;
use combine::{self as cmb, parser::char as chr, Parser, Stream};
use std::cell::RefCell;
use std::rc::Rc;

fn element<I: Stream<Token = char>, L: Logger>(
	logger: Rc<RefCell<L>>,
) -> impl Parser<I, Output = Rc<Node>> {
	let check = (ws::<I>(), string_parser::<I>(), ws::<I>());
	let l = logger.clone();

	let key = (cmb::look_ahead(check), value::<I, L>(logger.clone())).map(|(_, v)| v);
	(key, chr::char(':'), value(logger.clone())).map(move |(k, _, v)| {
		let NodeValue::Terminal(key) = k.value() else {
			unreachable!()
		};
		let TerminalNode::String(key) = key else {
			unreachable!()
		};

		let id = ObjectIdentity::try_from(key.as_str()).unwrap();
		v.set_identity(Identity::from(id.escaped()));

		l.borrow_mut()
			.write_verbose(&format!("key: {} value: {}", key.as_str(), v));

		v
	})
}

fn first<I: Stream<Token = char>, L: Logger>(
	logger: Rc<RefCell<L>>,
) -> impl Parser<I, Output = Rc<Node>> {
	element::<I, L>(logger.clone())
}

fn following<I: Stream<Token = char>, L: Logger>(
	logger: Rc<RefCell<L>>,
) -> impl Parser<I, Output = Vec<Rc<Node>>> {
	let tmp = (chr::char(','), element(logger.clone())).map(|(_, o)| o);
	cmb::many::<Vec<Rc<Node>>, I, _>(tmp)
}

fn contents<I: Stream<Token = char>, L: Logger>(
	logger: Rc<RefCell<L>>,
) -> impl Parser<I, Output = NodeValue> {
	let empty = ws::<I>().map(|_| NodeValue::Object(NonTerminalNode::new(Vec::new())));

	let l = logger.clone();
	let contents = (first::<I, L>(logger.clone()), following(logger.clone())).map(move |(a, b)| {
		let mut v = b;
		l.borrow_mut().write_verbose("foo");
		v.insert(0, a);

		NodeValue::Object(NonTerminalNode::new(v))
	});

	cmb::attempt(contents).or(empty)
}

pub fn object<I: Stream<Token = char>, L: Logger>(
	logger: Rc<RefCell<L>>,
) -> impl Parser<I, Output = NodeValue> {
	(
		chr::char('{'),
		contents::<I, L>(logger.clone()),
		chr::char('}'),
	)
		.map(|(_, c, _)| c)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::log::test_prelude::test_logger::MockLogger;
	use crate::syntax_node::test_prelude::*;

	fn gen_logger() -> Rc<RefCell<MockLogger>> {
		Rc::new(RefCell::new(MockLogger::new()))
	}

	fn generate_simple() -> String {
		fn g(scr: Vec<(&str, &str)>) -> String {
			let mut buffer = String::from('{');

			for (key, value) in scr {
				buffer.push_str(&format!(r##"{WS}"{key}"{WS}:{WS}{value}{WS},"##));
			}

			buffer.remove(buffer.len() - 1);
			buffer.push('}');
			buffer
		}

		g(vec![
			("int", "1"),
			("float", "1.0"),
			("string", r#""string""#),
			("null", "null"),
			("true", "true"),
			("false", "false"),
		])
	}

	#[test]
	fn single_object() {
		let mut parser = super::object::<&str, _>(gen_logger());
		let (a, r) = parser.parse(r#"{"foo":42.195}"#).unwrap();

		assert_eq!(r, "");

		let obj = a.extract_object().value();
		assert_eq!(obj.len(), 1);

		obj[0].identity().assert_key("foo");
		obj[0].value().extract_terminal().assert_float("42.195");
	}

	#[test]
	fn following() {
		let mut parser = super::following::<&str, _>(gen_logger());
		let (a, r) = parser.parse(r#","key1":1,"key2":true"#).unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 2);

		let piv = &a[0];
		piv.identity().assert_key("key1");
		piv.value().extract_terminal().assert_integer("1");

		let piv = &a[1];
		piv.identity().assert_key("key2");
		piv.value().extract_terminal().assert_true();
	}

	#[test]
	fn contents() {
		let mut parser = super::contents::<&str, _>(gen_logger());

		let (a, r) = parser.parse("").unwrap();
		assert_eq!(r, "");
		assert_eq!(0, a.extract_object().value().len());

		let (a, r) = parser.parse("   ").unwrap();
		assert_eq!(r, "");
		assert_eq!(0, a.extract_object().value().len());

		let (a, r) = parser.parse(r#"          "key" :null"#).unwrap();
		assert_eq!(r, "");
		let a = a.extract_object().value();
		assert_eq!(a.len(), 1);

		let piv = &a[0];
		piv.identity().assert_key("key");
		piv.value().extract_terminal().assert_null();

		let (a, r) = parser.parse(r#""key":null,"t":true,"f":false"#).unwrap();
		assert_eq!(r, "");
		let a = a.extract_object().value();
		assert_eq!(a.len(), 3);

		let piv = &a[0];
		piv.identity().assert_key("key");
		piv.value().extract_terminal().assert_null();

		let piv = &a[1];
		piv.identity().assert_key("t");
		piv.value().extract_terminal().assert_true();

		let piv = &a[2];
		piv.identity().assert_key("f");
		piv.value().extract_terminal().assert_false();
	}

	#[test]
	fn object() {
		let str = generate_simple();
		let mut parser = super::object::<&str, _>(gen_logger());

		let (_, r) = parser
			.parse(r#"{   "key"    :   42 ,"null":null}"#)
			.unwrap();
		assert_eq!(r, "");

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		let contents = act.extract_object().value();

		assert_eq!(contents.len(), 6);

		let piv = &contents[0];
		piv.identity().assert_key("int");
		piv.value().extract_terminal().assert_integer("1");

		let piv = &contents[1];
		piv.identity().assert_key("float");
		piv.value().extract_terminal().assert_float("1.0");

		let piv = &contents[2];
		piv.identity().assert_key("string");
		piv.value().extract_terminal().assert_string("string");

		let piv = &contents[3];
		piv.identity().assert_key("null");
		piv.value().extract_terminal().assert_null();

		let piv = &contents[4];
		piv.identity().assert_key("true");
		piv.value().extract_terminal().assert_true();

		let piv = &contents[5];
		piv.identity().assert_key("false");
		piv.value().extract_terminal().assert_false();
	}

	#[test]
	fn element() {
		let mut parser = super::element::<&str, _>(gen_logger());
		let (a, r) = parser.parse(r#""key":true"#).unwrap();
		assert_eq!(r, "");
		a.identity().assert_key("key");
		a.value().extract_terminal().assert_true();

		let (a, r) = parser.parse(r#"   "key"    :    true   "#).unwrap();
		assert_eq!(r, "");
		a.identity().assert_key("key");
		a.value().extract_terminal().assert_true();

		assert!(parser.parse("40:40").is_err())
	}
	#[test]
	fn empty() {
		let str = "{}";
		let mut parser = super::object::<&str, _>(gen_logger());

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.extract_object().value().len(), 0);

		let str = format!("{{{WS}}}");
		let mut parser = super::object::<&str, _>(gen_logger());
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		assert_eq!(act.extract_object().value().len(), 0);
	}

	#[test]
	fn invalid() {
		let str = "{50:50}";
		let mut parser = super::object::<&str, _>(gen_logger());

		assert!(parser.parse(str).is_err())
	}
}
