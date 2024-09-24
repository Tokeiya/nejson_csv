use super::string::string as string_parser;
use super::value::{value, ws};
use crate::syntax_node::prelude::*;
use combine::{
	self as cmb,
	parser::{self as psr, char as chr},
	Parser, Stream,
};

fn element<I: Stream<Token = char>>() -> impl Parser<I, Output = ObjectElement> {
	let key = (cmb::look_ahead(string_parser()), value::<I>()).map(|(_, v)| v);
	(key, chr::char(':'), value()).map(|(k, _, v)| ObjectElement::new(k, v))
}

pub fn object<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	let elements = (chr::char::<I>(','), element::<I>()).map(|(_, elem)| elem);
	let elements = cmb::many::<Vec<ObjectElement>, I, _>(elements);
	let elements = (element::<I>(), elements).map(|(a, b)| {
		let mut v = b;
		v.insert(0, a);
		NodeValue::Object(ObjectNode::new(v))
	});

	let empty = ws().map(|s| NodeValue::Object(ObjectNode::empty(s)));
	let contents = elements.or(empty);

	(chr::char('{'), contents, chr::char('}')).map(|(_, c, _)| c)
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
	fn object() {
		let str = generate_simple();
		let mut parser = super::object::<&str>();

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
