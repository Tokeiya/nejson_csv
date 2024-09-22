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

	#[test]
	fn array() {
		let mut parser = super::array::<&str>();

		let (a, r) = parser
			.parse(
				r#"[    10 ,20.41       ,"foo",
		null]"#,
			)
			.unwrap();

		assert_eq!("", r);

		let arr = a.value().extract_contents();
		assert_eq!(arr.len(), 4);
	}
}
