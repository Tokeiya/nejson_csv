use super::non_terminal_value_type::NonTerminalNodeType;
use super::object_element::ObjectElement;
use super::terminal_node::TerminalNode;

pub type ArrayNode = NonTerminalNode<TerminalNode>;
pub type ObjectNode = NonTerminalNode<ObjectElement>;

pub enum NonTerminalNodeValue<T> {
	Empty(String),
	Contents(Vec<T>),
}

pub struct NonTerminalNode<T> {
	node_type: NonTerminalNodeType,
	value: NonTerminalNodeValue<T>,
}

impl NonTerminalNode<TerminalNode> {
	pub fn new(value: Vec<TerminalNode>) -> Self {
		Self {
			node_type: NonTerminalNodeType::Array,
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty(white_space: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Array,
			value: NonTerminalNodeValue::Empty(white_space),
		}
	}
}

impl NonTerminalNode<ObjectElement> {
	pub fn new(value: Vec<ObjectElement>) -> Self {
		Self {
			node_type: NonTerminalNodeType::Object,
			value: NonTerminalNodeValue::Contents(value),
		}
	}

	pub fn empty(white_space: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Object,
			value: NonTerminalNodeValue::Empty(white_space),
		}
	}
}

impl<T> NonTerminalNode<T> {
	pub fn value(&self) -> &NonTerminalNodeValue<T> {
		&self.value
	}

	pub fn node_type(&self) -> NonTerminalNodeType {
		self.node_type
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
	use crate::syntax_node::prelude::TerminalNodeType;

	#[test]
	fn array_new() {
		let node = ArrayNode::new(vec![
			TerminalNode::new(TerminalNodeType::String, "foo".to_string()),
			TerminalNode::new(TerminalNodeType::Number, "42".to_string()),
		]);
		assert_eq!(node.node_type(), NonTerminalNodeType::Array);

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].assert_default_ws(TerminalNodeType::String, "foo");
		contents[1].assert_default_ws(TerminalNodeType::Number, "42");

		let node = ArrayNode::empty("space".to_string());
		node.value.assert_empty("space");
	}

	#[test]
	fn object_new() {
		let node = ObjectNode::new(vec![
			ObjectElement::new(
				TerminalNode::new(TerminalNodeType::String, "foo".to_string()),
				NodeValue::Terminal(TerminalNode::new(
					TerminalNodeType::Number,
					"42".to_string(),
				)),
			),
			ObjectElement::new(
				TerminalNode::new(TerminalNodeType::String, "bar".to_string()),
				NodeValue::Terminal(TerminalNode::new(
					TerminalNodeType::Number,
					"42.195".to_string(),
				)),
			),
		]);
		assert_eq!(node.node_type(), NonTerminalNodeType::Object);

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].assert_key("foo");
		contents[1]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Number, "42.195");

		let node = ObjectNode::empty("space".to_string());
		node.value.assert_empty("space");
	}

	#[test]
	fn value() {
		let node = ArrayNode::new(vec![
			TerminalNode::new(TerminalNodeType::String, "foo".to_string()),
			TerminalNode::new(TerminalNodeType::Number, "42".to_string()),
		]);
		let array = node.value().extract_contents();
		assert_eq!(array.len(), 2);

		array[0].assert_default_ws(TerminalNodeType::String, "foo");
		array[1].assert_default_ws(TerminalNodeType::Number, "42");

		let node = ObjectNode::new(vec![
			ObjectElement::new(
				TerminalNode::new(TerminalNodeType::String, "foo".to_string()),
				NodeValue::Terminal(TerminalNode::new(
					TerminalNodeType::Number,
					"42".to_string(),
				)),
			),
			ObjectElement::new(
				TerminalNode::new(TerminalNodeType::String, "bar".to_string()),
				NodeValue::Terminal(TerminalNode::new(
					TerminalNodeType::Number,
					"42.195".to_string(),
				)),
			),
		]);

		let object = node.value().extract_contents();
		assert_eq!(object.len(), 2);

		object[0].assert_key("foo");
		object[0]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Number, "42");

		object[1].assert_key("bar");
		object[1]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Number, "42.195");
	}
}
