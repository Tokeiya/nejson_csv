use crate::syntax_node::prelude::*;
use combine::{
	self as cmb,
	parser::{self as psr, char as chr},
	Parser, Stream,
};

pub fn object<I: Stream<Token = char>>() -> impl Parser<I, Output = ObjectNode> {
	chr::char('a').map(|_| todo!())
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
		let contents = act.value().extract_contents();

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
		act.value().assert_empty("");

		let str = format!("{{{WS}}}");
		let mut parser = super::object::<&str>();
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.value().assert_empty(WS);
	}
}
