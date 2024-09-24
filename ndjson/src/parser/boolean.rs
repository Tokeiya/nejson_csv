use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn boolean<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	chr::string::<I>("true")
		.or(chr::string::<I>("false"))
		.map(|str| {
			if str == "true" {
				NodeValue::Terminal(TerminalNode::True())
			} else if str == "false" {
				NodeValue::Terminal(TerminalNode::False())
			} else {
				unreachable!()
			}
		})
}

#[cfg(test)]
mod test {

	use super::*;
	#[test]
	fn boolean() {
		let mut parser = super::boolean::<&str>();
		let (a, rem) = parser.parse("true").unwrap();
		assert_eq!(rem, "");
		a.extract_terminal().assert_true();

		let (a, rem) = parser.parse("false").unwrap();
		assert_eq!(rem, "");
		a.extract_terminal().assert_false();
	}

	#[test]
	fn invalid() {
		let mut parser = super::boolean::<&str>();
		assert!(parser.parse("True").is_err());
		assert!(parser.parse("False").is_err())
	}
}
