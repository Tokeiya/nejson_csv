use super::non_terminal_node::NonTerminalNode;
use super::terminal_node::TerminalNode;
pub enum NodeValue {
	Terminal(TerminalNode),
	Array(NonTerminalNode),
	Object(NonTerminalNode),
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

		pub fn extract_array(&self) -> &NonTerminalNode {
			match self {
				NodeValue::Array(value) => value,
				_ => panic!("Expected array"),
			}
		}

		pub fn extract_object(&self) -> &NonTerminalNode {
			match self {
				NodeValue::Object(value) => value,
				_ => panic!("Expected object"),
			}
		}
	}
}
