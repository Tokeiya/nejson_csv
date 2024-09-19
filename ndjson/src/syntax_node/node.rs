use super::non_terminal_node::{ArrayNode, ObjectNode};
use super::terminal_node::TerminalNode;
pub enum Node {
	Terminal(TerminalNode),
	Array(ArrayNode),
	Object(ObjectNode),
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	use std::sync::LazyLock;

	impl Node {
		pub fn extract_terminal(&self) -> &TerminalNode {
			match self {
				Node::Terminal(value) => value,
				_ => panic!("Expected terminal"),
			}
		}

		pub fn extract_array(&self) -> &ArrayNode {
			match self {
				Node::Array(value) => value,
				_ => panic!("Expected array"),
			}
		}

		pub fn extract_object(&self) -> &ObjectNode {
			match self {
				Node::Object(value) => value,
				_ => panic!("Expected object"),
			}
		}
	}

	pub static WS: LazyLock<&'static str> = LazyLock::new(|| "\u{20}\u{09}\u{0A}\u{0D}");

	pub fn ws() -> String {
		WS.to_string()
	}
}
