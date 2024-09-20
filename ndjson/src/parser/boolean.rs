use crate::syntax_node::prelude::TerminalNodeType;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn boolean<I: Stream<Token = char>>() -> impl Parser<I, Output = (String, TerminalNodeType)> {
	chr::string::<I>("true")
		.or(chr::string::<I>("false"))
		.map(|str| {
			if str == "true" {
				(str.to_string(), TerminalNodeType::Boolean)
			} else if str == "false" {
				(str.to_string(), TerminalNodeType::Boolean)
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
		let ((v, t), rem) = parser.parse("true").unwrap();
		assert_eq!(rem, "");
		assert_eq!(&v, "true");
		assert_eq!(t, TerminalNodeType::Boolean);

		let ((v, t), rem) = parser.parse("false").unwrap();
		assert_eq!(rem, "");
		assert_eq!(&v, "false");
		assert_eq!(t, TerminalNodeType::Boolean);
	}

	#[test]
	fn invalid() {
		let mut parser = super::boolean::<&str>();
		assert!(parser.parse("True").is_err());
		assert!(parser.parse("False").is_err())
	}
}
