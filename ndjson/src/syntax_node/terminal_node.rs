use super::terminal_value_type::TerminalNodeType;

pub struct TerminalNode {
	node_type: TerminalNodeType,
	value: String,
	lead: String,
	trail: String,
}

impl TerminalNode {
	pub fn new(node_type: TerminalNodeType, value: String, lead: String, trail: String) -> Self {
		TerminalNode {
			node_type,
			value,
			lead,
			trail,
		}
	}

	pub fn lead(&self) -> &str {
		&self.lead
	}

	pub fn value(&self) -> &str {
		&self.value
	}

	pub fn trail(&self) -> &str {
		&self.trail
	}

	pub fn node_type(&self) -> TerminalNodeType {
		self.node_type
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::super::node::test_helper::WS;
	use super::*;

	impl TerminalNode {
		pub fn assert_lead(&self, expected: &str) {
			assert_eq!(&self.lead, expected);
			assert_eq!(self.lead(), expected);
		}

		pub fn assert_value(&self, expected: &str) {
			assert_eq!(&self.value, expected);
			assert_eq!(self.value(), expected);
		}

		pub fn assert_trail(&self, expected: &str) {
			assert_eq!(&self.trail, expected);
			assert_eq!(self.trail(), expected);
		}

		pub fn assert_default_ws(&self, expected_type: TerminalNodeType, expected_value: &str) {
			self.assert_lead(&WS);
			self.assert_trail(&WS);
			self.assert_value(expected_value);
			assert_eq!(self.node_type(), expected_type);
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
		for elem in TerminalNodeTypes::new() {
			let fixture = TerminalNode::new(
				elem,
				"value".to_string(),
				"lead".to_string(),
				"trail".to_string(),
			);

			assert_eq!(fixture.node_type(), elem);
		}
	}

	#[test]
	fn value() {
		let fixture = TerminalNode::new(
			TerminalNodeType::Boolean,
			"bool".to_string(),
			"lead".to_string(),
			"trail".to_string(),
		);

		assert_eq!(fixture.value(), "bool");
	}
}
