use crate::syntax_node::prelude::Node;
use combine as cmb;
use combine::parser;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn value<I: Stream<Token = char>>() -> impl Parser<I, Output = Node> {
	chr::char('a').map(|_| todo!())
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::prelude::TerminalNodeType;
	use crate::syntax_node::test_prelude::*;

	#[test]
	fn string() {
		let mut parser = value();
		let (v, r) = parser.parse(r#" "hello\n world" "#).unwrap();
		assert_eq!(r, "");
		let v = v.extract_terminal();
		assert_eq!(v.node_type(), TerminalNodeType::String);
		v.assert_lead(" ");
		v.assert_value(r#"hello\n world"#);
		v.assert_trail(" ")
	}
}
