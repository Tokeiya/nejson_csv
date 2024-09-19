use super::node::Node;
use super::non_terminal_value_type::NonTerminalNodeType;
use super::object_element::ObjectElement;
use super::terminal_node::TerminalNode;

pub enum NonTerminalNodeValue<T> {
	Empty(String),
	Contents(Vec<T>),
}

impl<T> From<String> for NonTerminalNodeValue<T> {
	fn from(value: String) -> Self {
		todo!()
	}
}

impl From<Vec<TerminalNode>> for NonTerminalNodeValue<TerminalNode> {
	fn from(value: Vec<TerminalNode>) -> Self {
		todo!()
	}
}

impl From<Vec<ObjectElement>> for NonTerminalNode<ObjectElement> {
	fn from(value: Vec<ObjectElement>) -> Self {
		todo!()
	}
}

pub struct NonTerminalNode<T> {
	node_type: NonTerminalNodeType,
	lead: String,
	value: NonTerminalNodeValue<T>,
	trail: String,
}
