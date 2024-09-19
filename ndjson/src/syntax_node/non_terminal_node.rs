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
	lead: String,
	value: NonTerminalNodeValue<T>,
	trail: String,
}

impl NonTerminalNode<TerminalNode> {
	pub fn new(value: Vec<TerminalNode>, lead: String, trail: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Array,
			lead,
			value: NonTerminalNodeValue::Contents(value),
			trail,
		}
	}

	pub fn empty(lead: String, white_space: String, trail: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Array,
			lead,
			value: NonTerminalNodeValue::Empty(white_space),
			trail,
		}
	}
}

impl NonTerminalNode<ObjectElement> {
	pub fn new(value: Vec<ObjectElement>, lead: String, trail: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Object,
			lead,
			value: NonTerminalNodeValue::Contents(value),
			trail,
		}
	}

	pub fn empty(lead: String, white_space: String, trail: String) -> Self {
		Self {
			node_type: NonTerminalNodeType::Object,
			lead,
			value: NonTerminalNodeValue::Empty(white_space),
			trail,
		}
	}
}

impl<T> NonTerminalNode<T> {
	pub fn lead(&self) -> &str {
		&self.lead
	}

	pub fn value(&self) -> &NonTerminalNodeValue<T> {
		&self.value
	}

	pub fn trail(&self) -> &str {
		&self.trail
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
	use super::super::node::test_helper::{ws, WS};
	use super::super::node::Node;
	use super::*;
	use crate::syntax_node::prelude::TerminalNodeType;

	#[test]
	fn array_new() {
		let node = ArrayNode::new(
			vec![
				TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
				TerminalNode::new(TerminalNodeType::Integer, "42".to_string(), ws(), ws()),
			],
			ws(),
			ws(),
		);
		assert_eq!(node.node_type(), NonTerminalNodeType::Array);
		assert_eq!(&node.lead, *WS);
		assert_eq!(&node.trail, *WS);

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].assert_default_ws(TerminalNodeType::String, "foo");
		contents[1].assert_default_ws(TerminalNodeType::Integer, "42");

		let node = ArrayNode::empty("lead".to_string(), "space".to_string(), "trail".to_string());
		assert_eq!(&node.lead, "lead");
		node.value.assert_empty("space");
		assert_eq!(&node.trail, "trail");
	}

	#[test]
	fn object_new() {
		let node = ObjectNode::new(
			vec![
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Integer,
						"42".to_string(),
						ws(),
						ws(),
					)),
				),
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "bar".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Float,
						"42.195".to_string(),
						ws(),
						ws(),
					)),
				),
			],
			ws(),
			ws(),
		);
		assert_eq!(node.node_type(), NonTerminalNodeType::Object);
		assert_eq!(&node.lead, *WS);
		assert_eq!(&node.trail, *WS);

		let contents = node.value().extract_contents();
		assert_eq!(contents.len(), 2);
		contents[0].assert_key("foo", *WS, *WS);
		contents[1]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Float, "42.195");

		let node = ObjectNode::empty("lead".to_string(), "space".to_string(), "trail".to_string());
		assert_eq!(&node.lead, "lead");
		node.value.assert_empty("space");
		assert_eq!(&node.trail, "trail");
	}

	#[test]
	fn value() {
		let node = ArrayNode::new(
			vec![
				TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
				TerminalNode::new(TerminalNodeType::Integer, "42".to_string(), ws(), ws()),
			],
			ws(),
			ws(),
		);
		let array = node.value().extract_contents();
		assert_eq!(array.len(), 2);

		array[0].assert_default_ws(TerminalNodeType::String, "foo");
		array[1].assert_default_ws(TerminalNodeType::Integer, "42");

		let node = ObjectNode::new(
			vec![
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Integer,
						"42".to_string(),
						ws(),
						ws(),
					)),
				),
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "bar".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Float,
						"42.195".to_string(),
						ws(),
						ws(),
					)),
				),
			],
			ws(),
			ws(),
		);

		let object = node.value().extract_contents();
		assert_eq!(object.len(), 2);

		object[0].assert_key("foo", *WS, *WS);
		object[0]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Integer, "42");

		object[1].assert_key("bar", *WS, *WS);
		object[1]
			.value()
			.extract_terminal()
			.assert_default_ws(TerminalNodeType::Float, "42.195");
	}

	#[test]
	fn lead_trail() {
		let node = ArrayNode::new(
			vec![
				TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
				TerminalNode::new(TerminalNodeType::Integer, "42".to_string(), ws(), ws()),
			],
			ws(),
			ws(),
		);
		assert_eq!(node.lead(), *WS);

		let node = ObjectNode::new(
			vec![
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "foo".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Integer,
						"42".to_string(),
						ws(),
						ws(),
					)),
				),
				ObjectElement::new(
					TerminalNode::new(TerminalNodeType::String, "bar".to_string(), ws(), ws()),
					Node::Terminal(TerminalNode::new(
						TerminalNodeType::Float,
						"42.195".to_string(),
						ws(),
						ws(),
					)),
				),
			],
			ws(),
			ws(),
		);
		assert_eq!(node.lead(), *WS);
		assert_eq!(node.trail(), *WS);
	}
}
