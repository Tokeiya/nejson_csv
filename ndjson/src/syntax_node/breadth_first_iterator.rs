use super::node::Node;
use std::collections::vec_deque::VecDeque;
use std::rc::Rc;
use crate::syntax_node::node_value::NodeValue;
use crate::syntax_node::test_prelude::*;
pub struct BreadthFirstIterator(VecDeque<Rc<Node>>);

impl BreadthFirstIterator {
	pub fn new(root: Rc<Node>) -> BreadthFirstIterator {
		let mut vec = VecDeque::new();
		vec.push_back(root);
		BreadthFirstIterator(vec)
	}
}
impl Iterator for BreadthFirstIterator {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()		
	}
}

#[cfg(test)]
mod test {
	use super::super::super::syntax_node::test_prelude::*;
	use super::*;
	use std::borrow::Borrow;

	#[test]
	fn new() {
		let root = node_helper::gen_sample();
		let fixture = BreadthFirstIterator::new(root.clone());
		assert!(std::ptr::eq(
			root.borrow() as *const Node,
			fixture.0[0].borrow() as *const Node
		))
	}

	#[test]
	fn breadth_first_iterator() {
		let root = node_helper::gen_sample();
		
		let mut iter= BreadthFirstIterator::new(root);

		for elem in iter {
			println!("{}",elem.full_qualified_name().text_expression())
		}
	}
}
