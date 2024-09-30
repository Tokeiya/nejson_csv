use std::cell::{Ref, RefCell};

pub enum NodeValue<'a> {
	Terminal(i32),
	NonTerminal(Vec<Node<'a>>),
}

pub struct Node<'a> {
	value: NodeValue<'a>,
	parent: RefCell<Option<&'a Node<'a>>>,
}

impl<'a> NodeValue<'a> {
	pub fn terminal_new(value: i32) -> Self {
		NodeValue::Terminal(value)
	}

	pub fn nonterminal_new(value: Vec<Node<'a>>) -> Self {
		NodeValue::NonTerminal(value)
	}
}

impl<'a> Node<'a> {
	pub fn new(value: NodeValue<'a>, parent: Option<&'a Node<'a>>) -> Self {
		Node {
			value,
			parent: RefCell::new(parent),
		}
	}

	pub fn value(&self) -> &NodeValue<'a> {
		&self.value
	}

	pub fn parent(&self) -> Ref<Option<&'a Node<'a>>> {
		self.parent.borrow()
	}

	pub fn set_parent(&self, parent: Option<&'a Node<'a>>) {
		self.parent.replace(parent);
	}
}
