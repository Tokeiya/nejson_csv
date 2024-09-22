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
		todo!()
	}

	#[test]
	fn empty() {
		todo!();
	}
}
