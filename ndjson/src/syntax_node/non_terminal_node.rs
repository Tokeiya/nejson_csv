use super::object_element::ObjectElement;
use super::prelude::*;

pub type ArrayNode = NonTerminalNode<Node>;
pub type ObjectNode = NonTerminalNode<ObjectElement>;

pub enum NonTerminalNodeValue<T> {
	Empty(String),
	Contents(Vec<T>),
}

pub struct NonTerminalNode<T> {
	value: NonTerminalNodeValue<T>,
}

impl NonTerminalNode<Node> {
	pub fn new(value: Vec<Node>) -> Self {
		Self {
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty(white_space: String) -> Self {
		Self {
			value: NonTerminalNodeValue::Empty(white_space),
		}
	}
}

impl NonTerminalNode<ObjectElement> {
	pub fn new(value: Vec<ObjectElement>) -> Self {
		Self {
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty(white_space: String) -> Self {
		Self {
			value: NonTerminalNodeValue::Empty(white_space),
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
		pub fn assert_empty(&self, expected: &str) {
			match self {
				NonTerminalNodeValue::Empty(value) => {
					assert_eq!(value, expected);
				}
				_ => panic!("Expected empty value"),
			}
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
	use crate::syntax_node::prelude::*;
	use crate::syntax_node::test_prelude::*;

	fn array_fixture() -> ArrayNode {
		let arr = vec![
			Node::new(
				NodeValue::Terminal(TerminalNode::String("foo".to_string())),
				ws(),
				ws(),
			),
			Node::new(
				NodeValue::Terminal(TerminalNode::Integer("42".to_string())),
				ws(),
				ws(),
			),
		];
		ArrayNode::new(arr)
	}

	#[test]
	fn array_new() {
		let node = array_fixture();

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].value().extract_terminal().assert_string("foo");
		contents[0].assert_trail(None);
		contents[0].assert_lead(None);

		contents[1].value().extract_terminal().assert_integer("42");
		contents[1].assert_trail(None);
		contents[1].assert_lead(None);

		let node = ArrayNode::empty("space".to_string());
		node.value.assert_empty("space");
	}

	#[test]
	fn object_new() {
		let node = ObjectNode::new(vec![
			ObjectElement::new(
				Node::new(
					NodeValue::Terminal(TerminalNode::String("foo".to_string())),
					ws(),
					ws(),
				),
				Node::new(
					NodeValue::Terminal(TerminalNode::Integer("42".to_string())),
					ws(),
					ws(),
				),
			),
			ObjectElement::new(
				Node::new(
					NodeValue::Terminal(TerminalNode::String("bar".to_string())),
					ws(),
					ws(),
				),
				Node::new(
					NodeValue::Terminal(TerminalNode::Float("42.195".to_string())),
					ws(),
					ws(),
				),
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

		let node = ObjectNode::empty("space".to_string());
		node.value.assert_empty("space");
	}

	#[test]
	fn value() {
		let node = array_fixture();
		let array = node.value().extract_contents();
		assert_eq!(array.len(), 2);

		array[0].assert_lead_trail(None, None);
		array[0].value().extract_terminal().assert_string("foo");

		array[1].assert_lead_trail(None, None);
		array[1].value().extract_terminal().assert_integer("42");

		let node = ObjectNode::new(vec![
			ObjectElement::new(
				Node::new(
					NodeValue::Terminal(TerminalNode::String("foo".to_string())),
					ws(),
					ws(),
				),
				Node::new(
					NodeValue::Terminal(TerminalNode::Integer("42".to_string())),
					ws(),
					ws(),
				),
			),
			ObjectElement::new(
				Node::new(
					NodeValue::Terminal(TerminalNode::String("bar".to_string())),
					ws(),
					ws(),
				),
				Node::new(
					NodeValue::Terminal(TerminalNode::Float("42.195".to_string())),
					ws(),
					ws(),
				),
			),
		]);

		let object = node.value().extract_contents();
		assert_eq!(object.len(), 2);

		object[0].value().assert_lead_trail(None, None);
		object[0].assert_key("foo");
		object[0]
			.value()
			.value()
			.extract_terminal()
			.assert_integer("42");

		object[1].value().assert_lead_trail(None, None);
		object[1].assert_key("bar");
		object[1]
			.value()
			.value()
			.extract_terminal()
			.assert_float("42.195");
	}
}
