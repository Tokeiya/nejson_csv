use crate::syntax_node::prelude::Node;
use std::rc::Rc;

pub struct Children(Rc<Node>);

impl From<Rc<Node>> for Children {
	fn from(value: Rc<Node>) -> Self {
		todo!()
	}
}

impl From<&Rc<Node>> for Children {
	fn from(value: &Rc<Node>) -> Self {
		todo!()
	}
}

impl Iterator for Children {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::super::test_prelude::node_helper::gen_sample;
	use super::*;
	use crate::syntax_node::prelude::*;
	use std::ptr::eq;

	fn assert_ref_equal<T>(l: &T, r: &T) {
		assert!(eq(l as *const T, r as *const T));
	}

	#[test]
	fn from_rc() {
		let root = gen_sample();
		let fixture = Children::from(root.clone());

		assert_ref_equal(&root, &fixture.0);
	}

	#[test]
	fn from_ref_rc() {
		let root = gen_sample();
		let fixture = Children::from(&root);

		assert_ref_equal(&root, &fixture.0);
	}

	#[test]
	fn terminal() {
		let root = Node::new(NodeValue::Terminal(TerminalNode::Null()));
		let mut fixture = Children::from(root.clone());

		for _ in 0..10 {
			assert!(fixture.next().is_some());
		}

		assert_eq!(Children::from(root).count(), 0);
	}

	#[test]
	fn arr_terminal() {
		todo!()
	}
}
