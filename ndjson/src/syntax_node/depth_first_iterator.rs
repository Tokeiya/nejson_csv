use super::node::Node;
use std::rc::Rc;

pub struct DepthFirstIterator {}

impl Iterator for DepthFirstIterator {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
