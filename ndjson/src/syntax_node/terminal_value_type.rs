pub enum TerminalNodeType {
	Boolean,
	Number,
	String,
	Null,
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	use std::sync::LazyLock;

	pub static TERMINAL_NODE_TYPES: LazyLock<[TerminalNodeType; 4]> = LazyLock::new(|| {
		[
			TerminalNodeType::Boolean,
			TerminalNodeType::Number,
			TerminalNodeType::String,
			TerminalNodeType::Null,
		]
	});

	#[cfg(test)]

	impl TerminalNodeType {
		#[cfg(test)]

		pub fn assert_boolean(&self) {
			assert!(matches!(self, TerminalNodeType::Boolean));
		}
		#[cfg(test)]

		pub fn assert_number(&self) {
			assert!(matches!(self, TerminalNodeType::Number));
		}
		#[cfg(test)]

		pub fn assert_string(&self) {
			assert!(matches!(self, TerminalNodeType::String));
		}
		#[cfg(test)]

		pub fn assert_null(&self) {
			assert!(matches!(self, TerminalNodeType::Null));
		}
	}
}
