use super::terminal_value_type::TerminalNodeType;

pub struct TerminalNode {
	node_type: TerminalNodeType,
	value: String,
	lead: String,
	trail: String,
}

impl TerminalNode {
	pub fn new(node_type: TerminalNodeType, value: String, lead: String, trail: String) -> Self {
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

		pub fn assert_default_ws(&self, expected_type: TerminalNodeType, expected_value: &str) {
			self.assert_lead(&WS);
			self.assert_trail(&WS);
			self.assert_value(expected_value);
			assert_eq!(self.node_type(), &expected_type);
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::test_prelude::*;
	use super::*;

	#[test]
	fn new() {
		for elem in TerminalNodeTypes::new() {
			let node = TerminalNode::new(elem, format!("{elem:?}"), ws(), ws());
			node.assert_default_ws(elem, &format!("{elem:?}"));
		}
	}

	#[test]
	fn lead() {
		let fixture = TerminalNode::new(
			TerminalNodeType::Boolean,
			"true".to_string(),
			"lead".to_string(),
			"trail".to_string(),
		);
		fixture.assert_lead("lead");
	}

	#[test]
	fn trail() {
		let fixture = TerminalNode::new(
			TerminalNodeType::Boolean,
			"true".to_string(),
			"lead".to_string(),
			"trail".to_string(),
		);
		fixture.assert_trail("trail");
	}

	#[test]
	fn node_type() {
		todo!()
	}

	#[test]
	fn value() {
		todo!();
	}
}
