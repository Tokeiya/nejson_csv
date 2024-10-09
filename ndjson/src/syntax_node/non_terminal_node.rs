use super::prelude::*;
use std::rc::Rc;

pub struct NonTerminalNode {
	value: Vec<Rc<Node>>,
}

impl NonTerminalNode {
	pub fn new(value: Vec<Rc<Node>>) -> Self {
		Self { value }
	}
}

impl NonTerminalNode {
	pub fn value(&self) -> &[Rc<Node>] {
		&self.value
	}
}
#[cfg(test)]
mod test {
	use super::super::node_value::NodeValue;
	use super::*;

	fn array_fixture() -> NonTerminalNode {
		let arr = vec![
			Node::new(NodeValue::Terminal(TerminalNode::String("foo".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string()))),
		];

		for (i, e) in arr.iter().enumerate() {
			e.set_identity(Identity::from(i))
		}

		NonTerminalNode::new(arr)
	}

	#[test]
	fn array_new() {
		let node = array_fixture();

		let contents = node.value();
		assert_eq!(contents.len(), 2);
		contents[0].value().extract_terminal().assert_string("foo");
		contents[1].value().extract_terminal().assert_integer("42");
	}

	fn fixture() -> NonTerminalNode {
		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Float(
				"42.195".to_string(),
			))),
		];

		vec[0].set_identity(Identity::from("foo"));
		vec[1].set_identity(Identity::from("bar"));

		NonTerminalNode::new(vec)
	}

	#[test]
	fn object_new() {
		let node = fixture();

		let contents = node.value();
		assert_eq!(contents.len(), 2);

		contents[0].identity().assert_key("foo");
		contents[0].value().extract_terminal().assert_integer("42");

		contents[1].identity().assert_key("bar");
		contents[1]
			.value()
			.extract_terminal()
			.assert_float("42.195");
	}

	#[test]
	fn value() {
		let node = array_fixture();
		let array = node.value();
		assert_eq!(array.len(), 2);

		array[0].value().extract_terminal().assert_string("foo");
		array[0].identity().assert_index(0);

		array[1].value().extract_terminal().assert_integer("42");
		array[1].identity().assert_index(1);

		let node = fixture();

		let object = node.value();
		assert_eq!(object.len(), 2);

		object[0].identity().assert_key("foo");
		object[0].value().extract_terminal().assert_integer("42");

		object[1].identity().assert_key("bar");
		object[1].value().extract_terminal().assert_float("42.195");
	}
}
