use super::empty_iterator::EmptyIterator;
use super::prelude::*;
use std::rc::Rc;

pub enum Direction {
	Breadth,
	Depth,
}

pub trait Traverse {
	fn children(&self) -> Children;
	fn descendants(&self, direction: Direction) -> Box<dyn Iterator<Item = &Rc<Node>>>;
}

impl Traverse for Rc<Node> {
	fn children(&self) -> Children {
		todo!()
	}

	fn descendants(&self, direction: Direction) -> Box<dyn Iterator<Item = &Rc<Node>>> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::super::test_prelude::*;
	use super::*;

	#[test]
	fn empty_children() {
		let fixture = Node::new(NodeValue::Terminal(TerminalNode::False()));
		let mut act = fixture.children();
		assert!(act.next().is_none());
	}

	#[test]
	fn arr_children() {
		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::Null())),
			Node::new(NodeValue::Terminal(TerminalNode::Null())),
			Node::new(NodeValue::Terminal(TerminalNode::Null())),
		];

		for (idx, elem) in vec.iter().enumerate() {
			elem.set_identity(Identity::Index(idx))
		}

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec)));

		for elem in arr.value().extract_array().value() {
			elem.set_parent(arr.clone())
		}

		for (idx, elem) in arr.children().enumerate() {
			elem.identity().assert_index(idx)
		}

		assert_eq!(arr.children().count(), 3);
	}

	#[test]
	fn obj_children() {
		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::True())),
			Node::new(NodeValue::Terminal(TerminalNode::Null())),
			Node::new(NodeValue::Terminal(TerminalNode::Null())),
		];

		for (idx, elem) in vec.iter().enumerate() {
			elem.set_identity(Identity::Key(idx.to_string()))
		}

		let obj = Node::new(NodeValue::Object(NonTerminalNode::new(vec)));
		for elem in obj.value().extract_object().value() {
			elem.set_parent(obj.clone())
		}

		for (idx, elem) in obj.children().enumerate() {
			elem.identity().assert_key(&idx.to_string())
		}

		assert_eq!(obj.children().count(), 3);
	}

	#[test]
	fn breadth_descendants() {
		let expected = vec![
			"Root",
			"Root::arr",
			"Root::arr::[0]",
			"Root::arr::[1]",
			"Root::arr::[0]0_0",
			"Root::arr::[0]1_1",
			"Root::arr::[1][0]",
			"Root::arr::[1][1]",
			"Root::arr::[1][2]",
			"Root::arr::[1][3]",
			"Root::arr::[1][2][0]",
			"Root::arr::[1][2][1]",
			"Root::arr::[1][3]1_0",
			"Root::arr::[1][3]1_1",
		];

		let root = node_helper::gen_sample();
		assert_eq!(root.children().count(), expected.len());

		for (act, exp) in root.descendants(Direction::Breadth).zip(expected.iter()) {
			assert_eq!(&act.full_qualified_name().text_expression(), exp)
		}
	}

	#[test]
	fn depth_descendants() {
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
		assert_eq!(root.children().count(), expected.len());

		for (act, exp) in root.descendants(Direction::Breadth).zip(expected.iter()) {
			assert_eq!(&act.full_qualified_name().text_expression(), exp)
		}
	}

	#[test]
	fn empty_descendants() {
		let root = Node::new(NodeValue::Terminal(TerminalNode::False()));

		assert_eq!(root.descendants(Direction::Breadth).count(), 0);
		assert_eq!(root.descendants(Direction::Depth).count(), 0);
	}
}
