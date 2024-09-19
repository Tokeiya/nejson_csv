use super::node::Node;
use super::terminal_node::TerminalNode;
pub struct ObjectElement {
	key: TerminalNode,
	value: Node,
}

impl ObjectElement {
	pub fn new(key: TerminalNode, value: Node) -> Self {
		todo!()
	}

	pub fn key(&self) -> &TerminalNode {
		todo!()
	}

	pub fn value(&self) -> &Node {
		todo!()
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
mod test {}
