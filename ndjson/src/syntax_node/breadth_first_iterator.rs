use super::node::Node;
use std::rc::Rc;

pub struct BreadthFirstIterator {}

impl Iterator for BreadthFirstIterator {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
