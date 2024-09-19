use super::node::Node;
use super::terminal_node::TerminalNode;
pub struct ObjectElement {
	key: TerminalNode,
	value: Node,
}

impl ObjectElement {
	pub fn new(key: TerminalNode, value: Node) -> Self {
		ObjectElement { key, value }
	}

	pub fn key(&self) -> &TerminalNode {
		&self.key
	}

	pub fn value(&self) -> &Node {
		&self.value
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::super::terminal_value_type::TerminalNodeType;
	use super::*;

	impl ObjectElement {
		pub fn assert_key(&self, value: &str, lead: &str, trail: &str) {
			let key = &self.key;
			assert_eq!(key.value(), value);
			assert_eq!(key.lead(), lead);
			assert_eq!(key.trail(), trail);
			assert_eq!(key.node_type(), TerminalNodeType::String);

			let key = &self.key();
			assert_eq!(key.value(), value);
			assert_eq!(key.lead(), lead);
			assert_eq!(key.trail(), trail);
			assert_eq!(key.node_type(), TerminalNodeType::String);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::prelude::TerminalNodeType;
	use crate::syntax_node::test_prelude::{ws, WS};

	#[test]
	fn new() {
		let key = TerminalNode::new(TerminalNodeType::String, "key".to_string(), ws(), ws());
		let value = Node::Terminal(TerminalNode::new(
			TerminalNodeType::String,
			"value".to_string(),
			ws(),
			ws(),
		));
		let object_element = ObjectElement::new(key, value);
		object_element.assert_key("key", *WS, *WS);
		object_element
			.value
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::String, "value");

		object_element
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::String, "value");
	}
}
