use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};
use std::cell::RefCell;
use std::rc::Rc;

pub fn null<I: Stream<Token = char>>() -> impl Parser<I, Output = NodeValue> {
	chr::string("null").map(|_| NodeValue::Terminal(TerminalNode::Null()))
}

#[cfg(test)]
mod test {
	use super::*;
	fn gen_logger() -> Rc<RefCell<Vec<String>>> {
		Rc::new(RefCell::new(Vec::new()))
	}
	#[test]
	fn null() {
		let mut parser = super::null::<&str>();
		let (a, rem) = parser.parse("null").unwrap();
		assert_eq!(rem, "");
		a.extract_terminal().assert_null();
	}

	#[test]
	fn invalid() {
		let mut parser = super::null::<&str>();
		assert!(parser.parse("Null").is_err());
		assert!(parser.parse("hoge").is_err())
	}
}
