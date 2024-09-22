use super::terminal_value_type::TerminalNodeType;

pub struct TerminalNode {
	node_type: TerminalNodeType,
	value: String,
}

impl TerminalNode {
	pub fn new(node_type: TerminalNodeType, value: String) -> Self {
		TerminalNode { node_type, value }
	}
	pub fn value(&self) -> &str {
		&self.value
	}
	pub fn node_type(&self) -> TerminalNodeType {
		self.node_type
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl TerminalNode {
		pub fn assert_value(&self, expected: &str) {
			assert_eq!(&self.value, expected);
			assert_eq!(self.value(), expected);
		}

		pub fn assert_default_ws(&self, expected_type: TerminalNodeType, expected_value: &str) {
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
			let node = TerminalNode::new(elem, format!("{elem:?}"));
			node.assert_default_ws(elem, &format!("{elem:?}"));
		}
	}

	#[test]
	fn node_type() {
		for elem in TerminalNodeTypes::new() {
			let fixture = TerminalNode::new(elem, "value".to_string());

			assert_eq!(fixture.node_type(), elem);
		}
	}

	#[test]
	fn value() {
		let fixture = TerminalNode::new(TerminalNodeType::Boolean, "bool".to_string());

		assert_eq!(fixture.value(), "bool");
	}
}
