use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};
use std::cell::RefCell;
use std::rc::Rc;

pub fn boolean<I: Stream<Token = char>>(
	logger: Rc<RefCell<Vec<String>>>,
) -> impl Parser<I, Output = NodeValue> {
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

	fn gen_logger() -> Rc<RefCell<Vec<String>>> {
		Rc::new(RefCell::new(Vec::new()))
	}

	#[test]
	fn boolean() {
		let mut parser = super::boolean::<&str>(gen_logger());
		let (a, rem) = parser.parse("true").unwrap();
		assert_eq!(rem, "");
		a.extract_terminal().assert_true();

		let (a, rem) = parser.parse("false").unwrap();
		assert_eq!(rem, "");
		a.extract_terminal().assert_false();
	}

	#[test]
	fn invalid() {
		let mut parser = super::boolean::<&str>(gen_logger());
		assert!(parser.parse("True").is_err());
		assert!(parser.parse("False").is_err())
	}
}
