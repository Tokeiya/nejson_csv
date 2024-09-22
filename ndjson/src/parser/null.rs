use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn null<I: Stream<Token = char>>() -> impl Parser<I, Output = TerminalNode> {
	chr::string("null").map(|_| TerminalNode::new(TerminalNodeType::Null, "null".to_string()))
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn null() {
		let mut parser = super::null::<&str>();
		let (a, rem) = parser.parse("null").unwrap();
		assert_eq!(rem, "");
		a.assert(TerminalNodeType::Null, "null");
	}

	#[test]
	fn invalid() {
		let mut parser = super::null::<&str>();
		assert!(parser.parse("Null").is_err());
		assert!(parser.parse("hoge").is_err())
	}
}
