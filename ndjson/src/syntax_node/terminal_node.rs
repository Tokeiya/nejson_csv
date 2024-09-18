use super::terminal_value_type::TerminalNodeType;

pub struct TerminalNode {
	node_type: TerminalNodeType,
	value: String,
	lead: String,
	trail: String,
}

impl TerminalNode {
	pub fn new(node_type: TerminalNode, value: String, lead: String, trail: String) {
		todo!()
	}

	pub fn lead(&self) -> &str {
		todo!()
	}

	pub fn value(&self) -> &str {
		todo!()
	}

	pub fn trail(&self) -> &str {
		todo!()
	}

	pub fn node_type(&self) -> &TerminalNodeType {
		todo!()
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::super::node::test_helper::WS;
	use super::*;

	impl TerminalNode {
		pub fn assert_lead(&self, expected: &str) {
			assert_eq!(&self.lead, expected);
		}

		pub fn assert_value(&self, expected: &str) {
			assert_eq!(&self.value, expected);
		}

		pub fn assert_trail(&self, expected: &str) {
			assert_eq!(&self.trail, expected);
		}

		pub fn assert_default_ws(&self, expected_value: &str) {
			self.assert_lead(&WS);
			self.assert_trail(&WS);
			self.assert_value(expected_value)
		}
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn new() {
		todo!();
	}

	#[test]
	fn lead() {
		todo!();
	}

	#[test]
	fn trail() {
		todo!();
	}

	#[test]
	fn node_type() {
		todo!();
	}
}
