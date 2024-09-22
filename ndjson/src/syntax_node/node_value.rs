use super::non_terminal_node::{ArrayNode, ObjectNode};
use super::terminal_node::TerminalNode;
pub enum NodeValue {
	Terminal(TerminalNode),
	Array(ArrayNode),
	Object(ObjectNode),
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl NodeValue {
		pub fn extract_terminal(&self) -> &TerminalNode {
			match self {
				NodeValue::Terminal(value) => value,
				_ => panic!("Expected terminal"),
			}
		}

		pub fn extract_array(&self) -> &ArrayNode {
			match self {
				NodeValue::Array(value) => value,
				_ => panic!("Expected array"),
			}
		}

		pub fn extract_object(&self) -> &ObjectNode {
			match self {
				NodeValue::Object(value) => value,
				_ => panic!("Expected object"),
			}
		}
	}
}
