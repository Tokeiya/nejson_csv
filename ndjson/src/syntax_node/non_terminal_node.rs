use super::object_element::ObjectElement;
use super::prelude::*;
use std::rc::Rc;

pub type ArrayNode = NonTerminalNode<Rc<Node>>;
pub type ObjectNode = NonTerminalNode<ObjectElement>;

pub enum NonTerminalNodeValue<T> {
	Empty,
	Contents(Vec<T>),
}

pub struct NonTerminalNode<T> {
	value: NonTerminalNodeValue<T>,
}

impl NonTerminalNode<Rc<Node>> {
	pub fn new(value: Vec<Rc<Node>>) -> Self {
		Self {
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty() -> Self {
		Self {
			value: NonTerminalNodeValue::Empty,
		}
	}
}

impl NonTerminalNode<ObjectElement> {
	pub fn new(value: Vec<ObjectElement>) -> Self {
		Self {
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty() -> Self {
		Self {
			value: NonTerminalNodeValue::Empty,
		}
	}
}

impl<T> NonTerminalNode<T> {
	pub fn value(&self) -> &NonTerminalNodeValue<T> {
		&self.value
	}
}
#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl<T> NonTerminalNodeValue<T> {
		pub fn assert_empty(&self) {
			let NonTerminalNodeValue::Empty = self else {
				unreachable!()
			};
		}

		pub fn extract_contents(&self) -> &[T] {
			match self {
				NonTerminalNodeValue::Contents(value) => value,
				_ => panic!("Expected contents"),
			}
		}
	}
}
#[cfg(test)]
mod test {
	use super::super::node_value::NodeValue;
	use super::*;

	fn array_fixture() -> ArrayNode {
		let arr = vec![
			Node::new(NodeValue::Terminal(TerminalNode::String("foo".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string()))),
		];

		for (i, e) in arr.iter().enumerate() {
			e.set_identity(Identity::from(i))
		}

		ArrayNode::new(arr)
	}

	#[test]
	fn array_new() {
		let node = array_fixture();

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].value().extract_terminal().assert_string("foo");

		contents[1].value().extract_terminal().assert_integer("42");

		let node = ArrayNode::empty();
		node.value.assert_empty();
	}

	#[test]
	fn object_new() {
		let node = ObjectNode::new(vec![
			ObjectElement::new(
				Node::new(NodeValue::Terminal(TerminalNode::String("foo".to_string()))),
				Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string()))),
			),
			ObjectElement::new(
				Node::new(NodeValue::Terminal(TerminalNode::String("bar".to_string()))),
				Node::new(NodeValue::Terminal(TerminalNode::Float(
					"42.195".to_string(),
				))),
			),
		]);

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].assert_key("foo");

		contents[0]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("42");

		contents[1].assert_key("bar");

		contents[1]
			.value()
			.value()
			.extract_terminal()
			.assert_float("42.195");

		let node = ObjectNode::empty();
		node.value.assert_empty();
	}

	#[test]
	fn value() {
		let node = array_fixture();
		let array = node.value().extract_contents();
		assert_eq!(array.len(), 2);

		array[0].value().extract_terminal().assert_string("foo");
		array[0].identity().assert_index(0);

		array[1].value().extract_terminal().assert_integer("42");
		array[1].identity().assert_index(1);

		let node = ObjectNode::new(vec![
			ObjectElement::new(
				Node::new(NodeValue::Terminal(TerminalNode::String("foo".to_string()))),
				Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string()))),
			),
			ObjectElement::new(
				Node::new(NodeValue::Terminal(TerminalNode::String("bar".to_string()))),
				Node::new(NodeValue::Terminal(TerminalNode::Float(
					"42.195".to_string(),
				))),
			),
		]);

		let object = node.value().extract_contents();
		assert_eq!(object.len(), 2);

		object[0].assert_key("foo");
		object[0]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("42");

		object[1].assert_key("bar");
		object[1]
			.value()
			.value()
			.extract_terminal()
			.assert_float("42.195");
	}
}
