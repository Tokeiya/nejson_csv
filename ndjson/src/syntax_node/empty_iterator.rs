use crate::syntax_node::node::Node;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct EmptyIterator;

impl Iterator for EmptyIterator {
	type Item = Rc<Node>;
	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn next() {
		let mut act = EmptyIterator;

		assert!(matches!(act.next(), None));
		assert!(matches!(act.next(), None));
	}
}
