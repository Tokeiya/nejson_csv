use super::value::value;
use crate::syntax_node::prelude::*;
use combine::{
	self as cmb,
	parser::{self as psr, char as chr},
	Parser, Stream,
};
pub fn array<I: Stream<Token = char>>() -> impl Parser<I, Output = ArrayNode> {
	chr::char('a').map(|_| todo!())
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
	fn array() {
		let str = generate_array(vec![r#""rust""#, "42", "null", "true", "false", "42.195"]);
		let mut parser = super::array::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");

		let act = act.value().extract_contents();
		assert_eq!(act.len(), 6);

		let piv = &act[0];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("rust");
		piv.value().extract_terminal().node_type().assert_string();

		let piv = &act[1];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("42");
		piv.value().extract_terminal().node_type().assert_integer();

		let piv = &act[2];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("null");
		piv.value().extract_terminal().node_type().assert_null();

		let piv = &act[3];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("true");
		piv.value().extract_terminal().node_type().assert_boolean();

		let piv = &act[4];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("false");
		piv.value().extract_terminal().node_type().assert_boolean();

		let piv = &act[5];
		piv.assert_lead_trail(None, None);
		piv.value().extract_terminal().assert_value("42.195");
		piv.value().extract_terminal().node_type().assert_float();
	}

	#[test]
	fn empty_array() {
		let str = "[]";
		let mut parser = super::array::<&str>();

		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.value().assert_empty("");

		let str = format!("[{WS}]");
		let mut parser = super::array::<&str>();
		let (act, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		act.value().assert_empty(WS);
	}
}
