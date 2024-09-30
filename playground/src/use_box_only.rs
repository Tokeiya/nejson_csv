use std::cell::RefCell;

pub enum Value {
	Terminal(i32),
	NonTerminal(),
}

pub struct Node<'a> {
	value: Value,
	parent: Option<&'a Node<'a>>,
}
