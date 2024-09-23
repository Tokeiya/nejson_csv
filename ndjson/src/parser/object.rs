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
	use crate::syntax_node::prelude::*;
	use crate::syntax_node::test_prelude::*;
	fn generate(scr: Vec<(&str, &str)>) -> String {
		let mut buffer = String::from('{');

		for (key, value) in scr {
			buffer.push_str(&format!(r##"{WS}"{key}"{WS}:{WS}{value}{WS},"##));
		}

		buffer.remove(buffer.len() - 1);
		buffer.push('}');
		buffer
	}

	#[test]
	fn object() {
		let str = generate(vec![
			("rust", r#""rust""#),
			("int", "42"),
			("null", "null"),
			("true", "true"),
			("false", "false"),
			("float", "42.195"),
		]);
		let mut parser = super::object::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");

		let act = act.value().extract_contents();
		assert_eq!(act.len(), 6);

		let piv = &act[0];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("rust");
		piv.value().extract_terminal().assert_string("rust");

		let piv = &act[1];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("int");
		piv.value().extract_terminal().assert_integer("42");

		let piv = &act[2];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("null");
		piv.value().extract_terminal().assert_null();

		let piv = &act[3];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("true");
		piv.value().extract_terminal().assert_true();

		let piv = &act[4];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("false");
		piv.value().extract_terminal().assert_false();

		let piv = &act[5];
		piv.assert_lead_trail(None, None);
		piv.key().assert_string("float");
		piv.value().extract_terminal().assert_float("42.195");
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
