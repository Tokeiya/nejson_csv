use super::node::Node;
use std::rc::Rc;

pub struct SingleIterator<'a>(&'a Rc<Node>, bool);

impl<'a> SingleIterator<'a> {
	pub fn new(node: &'a Rc<Node>) -> Self {
		SingleIterator(node, false)
	}
}

impl<'a> Iterator for SingleIterator<'a> {
	type Item = &'a Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.1 {
			None
		} else {
			self.1 = true;
			Some(self.0)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::prelude::*;

	#[test]
	fn new() {
		let node = Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string())));
		let fixture = SingleIterator::new(&node);

		assert_eq!(fixture.1, false);
		fixture.0.value().extract_terminal().assert_integer("42");
	}

	#[test]
	fn next() {
		let node = Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string())));
		let mut fixture = SingleIterator::new(&node);

		let act = fixture.next().unwrap();
		act.value().extract_terminal().assert_integer("42");

		assert!(matches!(fixture.next(), None));
		assert!(matches!(fixture.next(), None));
	}
}
