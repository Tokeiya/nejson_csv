use crate::syntax_node::prelude::*;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn null<I: Stream<Token = char>>() -> impl Parser<I, Output = (String, TerminalNodeType)> {
	chr::string("null").map(|_| ("null".to_string(), TerminalNodeType::Null))
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn null() {
		let mut parser = super::null::<&str>();
		let ((v, t), rem) = parser.parse("null").unwrap();
		assert_eq!(rem, "");
		assert_eq!(&v, "null");
		assert_eq!(t, TerminalNodeType::Null);
	}

	#[test]
	fn invalid() {
		let mut parser = super::null::<&str>();
		assert!(parser.parse("Null").is_err());
		assert!(parser.parse("hoge").is_err())
	}
}
