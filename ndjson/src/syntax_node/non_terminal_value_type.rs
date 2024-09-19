#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod test {
	use super::test_helper::*;
	use super::*;
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", NonTerminalNodeType::Array), "Array");
		assert_eq!(format!("{:?}", NonTerminalNodeType::Object), "Object");
	}

	#[test]
	fn copy() {
		let fixture = NonTerminalNodeType::Array;
		let copied = fixture;

		fixture.assert_array();
		copied.assert_array();
	}

	#[test]
	fn clone() {
		let fixture = NonTerminalNodeType::Object;
		let cloned = fixture.clone();

		fixture.assert_object();
		cloned.assert_object();
	}

	#[test]
	fn eq() {
		let fixture = NonTerminalNodeType::Array;
		assert_eq!(fixture, NonTerminalNodeType::Array);

		let fixture = NonTerminalNodeType::Object;
		assert_eq!(fixture, NonTerminalNodeType::Object);

		let fixture = NonTerminalNodeType::Array;
		assert_ne!(fixture, NonTerminalNodeType::Object);

		let fixture = NonTerminalNodeType::Object;
		assert_ne!(fixture, NonTerminalNodeType::Array);
	}
}
