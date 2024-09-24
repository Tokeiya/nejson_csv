use super::node_value::NodeValue;
use super::prelude::*;
use super::terminal_node::TerminalNode;

pub struct ObjectElement {
	key: Node,
	value: Node,
}

impl ObjectElement {
	pub fn new(key: Node, value: Node) -> Self {
		ObjectElement { key, value }
	}
	pub fn key(&self) -> &Node {
		&self.key
	}
	pub fn value(&self) -> &Node {
		&self.value
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ObjectElement {
		pub fn assert_key(&self, value: &str) {
			let key = &self.key;
			key.assert_lead_trail(None, None);
			key.value().extract_terminal().assert_string(value);

			let key = &self.key();
			key.value().extract_terminal().assert_string(value);
			key.assert_lead_trail(None, None);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::test_prelude::ws;

	#[test]
	fn new() {
		let key = Node::new(
			NodeValue::Terminal(TerminalNode::String("key".to_string())),
			ws(),
			ws(),
		);
		let value = Node::new(
			NodeValue::Terminal(TerminalNode::String("value".to_string())),
			ws(),
			ws(),
		);

		value.assert_lead_trail(None, None);

		let object_element = ObjectElement::new(key, value);
		object_element.assert_key("key");
		object_element
			.value
			.value()
			.extract_terminal()
			.assert_string("value");

		object_element
			.value()
			.value()
			.extract_terminal()
			.assert_string("value");
	}
}
