use std::cell::RefCell;

pub enum NodeValue<'a> {
	Terminal(i32),
	NonTerminal(RefCell<Vec<Node<'a>>>),
}

pub struct Node<'a> {
	value: NodeValue<'a>,
	parent: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
	pub fn new_terminal(value: i32, parent: Option<&'a Node<'a>>) -> Self {
		let v = NodeValue::Terminal(value);
		Node { value: v, parent }
	}

	pub fn new_nonterminal(parent: Option<&'a Node<'a>>) -> Self {
		let v = NodeValue::NonTerminal(RefCell::new(Vec::new()));
		Node { value: v, parent }
	}

	pub fn add_child(&self, child: Node<'a>) {
		if let NodeValue::NonTerminal(ref c) = self.value {
			let mut c = c.borrow_mut();
			c.push(child);
		} else {
			panic!()
		}
	}
}
