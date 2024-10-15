use crate::syntax_node::node_value::NodeValue;
use crate::syntax_node::prelude::Node;
use std::rc::Rc;

pub struct Children(Rc<Node>, usize);

impl From<Rc<Node>> for Children {
	fn from(value: Rc<Node>) -> Self {
		Children(value, 0)
	}
}

impl From<&Rc<Node>> for Children {
	fn from(value: &Rc<Node>) -> Self {
		Children(value.clone(), 0)
	}
}

impl Iterator for Children {
	type Item = Rc<Node>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.0.value() {
			NodeValue::Terminal(_) => None,
			NodeValue::Array(ary) => {
				todo!()
			}
			NodeValue::Object(_) => {
				todo!()
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::test_prelude::node_helper::{array, gen_sample, obj};
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
	fn arr_value() {
		let root = array();
		let mut fixture = Children::from(root.clone());

		for (idx, elem) in fixture.enumerate() {
			elem.identity().assert_index(idx);
		}

		assert_eq!(Children::from(&root).count(), 5);
	}

	#[test]
	fn obj_value() {
		let root = obj();

		for (idx, elem) in Children::from(&root).enumerate() {
			elem.identity().assert_key(&idx.to_string())
		}

		assert_eq!(Children::from(&root).count(), 5);
	}
}
