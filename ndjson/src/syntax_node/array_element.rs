use super::prelude::*;
use std::rc::Rc;
pub struct ArrayElement {
	index: usize,
	value: Rc<Node>,
}

impl ArrayElement {
	pub fn new(index: usize, value: Rc<Node>) -> ArrayElement {
		ArrayElement { index, value }
	}

	pub fn index(&self) -> usize {
		self.index
	}

	pub fn value(&self) -> &Node {
		&self.value
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ArrayElement {
		pub fn assert_index(&self, index: usize) {
			assert_eq!(index, self.index)
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::prelude::*;

	#[test]
	fn new() {
		let tmp = TerminalNode::Null();
		let tmp = NodeValue::Terminal(tmp);
		let tmp = Node::new(tmp);
		let fixture = ArrayElement::new(42, tmp);

		assert_eq!(fixture.index, 42);
		fixture.value.value().extract_terminal().assert_null()
	}

	#[test]
	fn index() {
		let tmp = TerminalNode::Null();
		let tmp = NodeValue::Terminal(tmp);
		let tmp = Node::new(tmp);

		let fixture = ArrayElement::new(42, tmp);
		fixture.assert_index(42);
	}

	#[test]
	fn value() {
		let tmp = TerminalNode::Null();
		let tmp = NodeValue::Terminal(tmp);
		let tmp = Node::new(tmp);

		let fixture = ArrayElement::new(42, tmp);
		fixture.value().value().extract_terminal().assert_null()
	}
}
