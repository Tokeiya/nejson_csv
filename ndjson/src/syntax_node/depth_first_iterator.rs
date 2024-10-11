use super::node::Node;
use crate::syntax_node::node_value::NodeValue;
use std::rc::Rc;

pub struct DepthFirstIterator(Vec<Rc<Node>>);

impl DepthFirstIterator {
	pub fn new(root: Rc<Node>) -> DepthFirstIterator {
		DepthFirstIterator(vec![root])
	}
}

impl Iterator for DepthFirstIterator {
	type Item = Rc<Node>;
	fn next(&mut self) -> Option<Self::Item> {
		let piv = self.0.pop()?;

		match piv.value() {
			NodeValue::Terminal(_) => {}
			NodeValue::Array(arr) => {
				for elem in arr.value().iter().rev() {
					self.0.push(elem.clone());
				}
			}
			NodeValue::Object(obj) => {
				for elem in obj.value().iter().rev() {
					self.0.push(elem.clone());
				}
			}
		}

		Some(piv)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::test_prelude::*;
	use std::borrow::Borrow;
	#[test]
	fn new() {
		let root = node_helper::gen_sample();

		let fixture = DepthFirstIterator::new(root.clone());

		assert!(std::ptr::eq(
			root.borrow() as *const Node,
			fixture.0[0].borrow() as *const Node
		));
	}

	#[test]
	fn depth_first_iterator() {
		let expected = vec![
			"Root",
			"Root::arr",
			"Root::arr::[0]",
			"Root::arr::[0]0_0",
			"Root::arr::[0]1_1",
			"Root::arr::[1]",
			"Root::arr::[1][0]",
			"Root::arr::[1][1]",
			"Root::arr::[1][2]",
			"Root::arr::[1][2][0]",
			"Root::arr::[1][2][1]",
			"Root::arr::[1][3]",
			"Root::arr::[1][3]1_0",
			"Root::arr::[1][3]1_1",
		];

		let root = node_helper::gen_sample();
		let fixture = DepthFirstIterator::new(root.clone());

		let mut cnt = 0usize;
		for elem in fixture {
			assert_eq!(expected[cnt], elem.full_qualified_name().text_expression());
			cnt += 1;
		}

		assert_eq!(cnt, expected.len());
	}
}
