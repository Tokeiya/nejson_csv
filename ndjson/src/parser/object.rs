use super::string::string as string_parser;
use super::value::{value, ws};
use crate::syntax_node::prelude::*;
use combine::{
	self as cmb,
	parser::{self as psr, char as chr},
	Parser, Stream,
};

fn element<I: Stream<Token = char>>() -> impl Parser<I, Output = ObjectElement> {
	let check = (ws::<I>(), string_parser::<I>(), ws::<I>());

	let key = (cmb::look_ahead(check), value::<I>()).map(|(_, v)| v);
	(key, chr::char(':'), value()).map(|(k, _, v)| ObjectElement::new(k, v))
}

fn first<I: Stream<Token = char>>() -> impl Parser<I, Output = ObjectElement> {
	element::<I>()
}

fn following<I: Stream<Token = char>>() -> impl Parser<I, Output = Vec<ObjectElement>> {
	let tmp = (chr::char(','), element()).map(|(_, o)| o);
	cmb::many::<Vec<ObjectElement>, I, _>(tmp)
}

fn contents<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	let empty = ws::<I>().map(|s| NodeValue::Object(ObjectNode::empty(s)));

	let contents = (first::<I>(), following()).map(|(a, b)| {
		let mut v = b;
		v.insert(0, a);

		NodeValue::Object(ObjectNode::new(v))
	});

	cmb::attempt(contents).or(empty)
}

pub fn object<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	(chr::char('{'), contents::<I>(), chr::char('}')).map(|(_, c, _)| c)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::prelude::*;
	use crate::syntax_node::test_prelude::*;

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
		let mut parser = super::object::<&str>();
		let (a, r) = parser.parse(r#"{"foo":42.195}"#).unwrap();

		assert_eq!(r, "");

		let obj = a.extract_object().value().extract_contents();
		assert_eq!(obj.len(), 1);

		obj[0].key().value().extract_terminal().assert_string("foo");
		obj[0]
			.value()
			.value()
			.extract_terminal()
			.assert_float("42.195");
	}

	#[test]
	fn following() {
		let mut parser = super::following::<&str>();
		let (a, r) = parser.parse(r#","key1":1,"key2":true"#).unwrap();
		assert_eq!(r, "");
		assert_eq!(a.len(), 2);

		let piv = &a[0];
		piv.key().value().extract_terminal().assert_string("key1");
		piv.value().value().extract_terminal().assert_integer("1");

		let piv = &a[1];
		piv.key().value().extract_terminal().assert_string("key2");
		piv.value().value().extract_terminal().assert_true();
	}

	#[test]
	fn contents() {
		let mut parser = super::contents::<&str>();

		let (a, r) = parser.parse("").unwrap();
		assert_eq!(r, "");
		a.extract_object().value().assert_empty("");

		let (a, r) = parser.parse("   ").unwrap();
		assert_eq!(r, "");
		a.extract_object().value().assert_empty("   ");

		let (a, r) = parser.parse(r#"          "key" :null"#).unwrap();
		assert_eq!(r, "");
		let a = a.extract_object().value().extract_contents();
		assert_eq!(a.len(), 1);

		let piv = &a[0];
		piv.key().value().extract_terminal().assert_string("key");
		piv.value().value().extract_terminal().assert_null();

		let (a, r) = parser.parse(r#""key":null,"t":true,"f":false"#).unwrap();
		assert_eq!(r, "");
		let a = a.extract_object().value().extract_contents();
		assert_eq!(a.len(), 3);

		let piv = &a[0];
		piv.key().value().extract_terminal().assert_string("key");
		piv.value().value().extract_terminal().assert_null();

		let piv = &a[1];
		piv.key().value().extract_terminal().assert_string("t");
		piv.value().value().extract_terminal().assert_true();

		let piv = &a[2];
		piv.key().value().extract_terminal().assert_string("f");
		piv.value().value().extract_terminal().assert_false();
	}

	#[test]
	fn object() {
		let str = generate_simple();
		let mut parser = super::object::<&str>();

		let (a, r) = parser
			.parse(r#"{   "key"    :   42 ,"null":null}"#)
			.unwrap();
		assert_eq!(r, "");

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		let contents = act.extract_object().value().extract_contents();

		assert_eq!(contents.len(), 6);

		let piv = &contents[0];
		piv.key().value().extract_terminal().assert_string("int");
		piv.value().value().extract_terminal().assert_integer("1");
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);

		let piv = &contents[1];
		piv.key().value().extract_terminal().assert_string("float");
		piv.value().value().extract_terminal().assert_float("1.0");
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);

		let piv = &contents[2];
		piv.key().value().extract_terminal().assert_string("string");
		piv.value()
			.value()
			.extract_terminal()
			.assert_string("string");
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);

		let piv = &contents[3];
		piv.key().value().extract_terminal().assert_string("null");
		piv.value().value().extract_terminal().assert_null();
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);

		let piv = &contents[4];
		piv.key().value().extract_terminal().assert_string("true");
		piv.value().value().extract_terminal().assert_true();
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);

		let piv = &contents[5];
		piv.key().value().extract_terminal().assert_string("false");
		piv.value().value().extract_terminal().assert_false();
		piv.value().assert_lead_trail(None, None);
		piv.key().assert_lead_trail(None, None);
	}

	#[test]
	fn element() {
		let mut parser = super::element::<&str>();
		let (a, r) = parser.parse(r#""key":true"#).unwrap();
		assert_eq!(r, "");
		a.key().value().extract_terminal().assert_string("key");
		a.value().value().extract_terminal().assert_true();

		let (a, r) = parser.parse(r#"   "key"    :    true   "#).unwrap();
		assert_eq!(r, "");
		a.key().value().extract_terminal().assert_string("key");
		a.value().value().extract_terminal().assert_true();

		assert!(parser.parse("40:40").is_err())
	}
	#[test]
	fn empty() {
		let str = "{}";
		let mut parser = super::object::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.extract_object().value().assert_empty("");

		let str = format!("{{{WS}}}");
		let mut parser = super::object::<&str>();
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.extract_object().value().assert_empty(WS);
	}

	#[test]
	fn invalid() {
		let str = "{50:50}";
		let mut parser = super::object::<&str>();

		assert!(parser.parse(str).is_err())
	}
}
