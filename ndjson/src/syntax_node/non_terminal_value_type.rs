pub enum NonTerminalNodeType {
	Array,
	Object,
}

#[cfg(test)]
pub mod test_helper {
	use super::NonTerminalNodeType;
	use std::sync::LazyLock;

	pub static NON_TERMINAL_NODE_TYPES: LazyLock<[NonTerminalNodeType; 2]> =
		LazyLock::new(|| [NonTerminalNodeType::Array, NonTerminalNodeType::Object]);
	impl NonTerminalNodeType {
		pub fn assert_array(&self) {
			assert!(matches!(self, NonTerminalNodeType::Array));
		}

		pub fn assert_object(&self) {
			assert!(matches!(self, NonTerminalNodeType::Object));
		}
	}
}
