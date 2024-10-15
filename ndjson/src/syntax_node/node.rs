use super::depth_first_iterator::DepthFirstIterator;
use super::identity::Identity;
use super::node_value::NodeValue;
use super::prelude::*;
use crate::syntax_node::breadth_first_iterator::BreadthFirstIterator;
use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

pub enum Direction {
	Depth,
	Breadth,
}

pub struct Node {
	value: NodeValue,
	identity: RefCell<Identity>,
	parent: RefCell<Weak<Node>>,
}

impl Node {
	pub fn new(value: NodeValue) -> Rc<Self> {
		Rc::new(Self {
			value,
			parent: RefCell::new(Weak::default()),
			identity: RefCell::new(Identity::Undefined),
		})
	}

	pub fn parent(&self) -> Option<Rc<Node>> {
		self.parent.borrow().upgrade()
	}

	pub fn set_parent(&self, parent: Rc<Node>) {
		self.parent.replace(Rc::downgrade(&parent));
	}

	pub fn identity(&self) -> Ref<Identity> {
		self.identity.borrow()
	}

	pub fn set_identity(&self, identity: Identity) {
		self.identity.replace(identity);
	}

	pub fn value(&self) -> &NodeValue {
		&self.value
	}

	pub fn full_qualified_name(&self) -> FullQualifiedName {
		let mut vec = Vec::new();
		self.collect_full_qualified_name(&mut vec);
		FullQualifiedName::from(vec)
	}

	fn collect_full_qualified_name(&self, vec: &mut Vec<Identity>) {
		if let Some(p) = self.parent() {
			p.collect_full_qualified_name(vec);
		}

		vec.push(self.identity().clone());
	}

	pub fn children(&self) -> Box<dyn Iterator<Item = &Rc<Node>> + '_> {
		match self.value() {
			NodeValue::Terminal(_) => Box::new(std::iter::empty()),
			NodeValue::Object(obj) => Box::new(obj.value().iter()),
			NodeValue::Array(arr) => Box::new(arr.value().iter()),
		}
	}

	pub fn descendants(&self, direction: Direction) -> Box<dyn Iterator<Item = &'_ Rc<Node>> + '_> {
		match direction {
			Direction::Depth => match self.value() {
				NodeValue::Terminal(_) => Box::new(std::iter::empty()),
				NodeValue::Array(ary) => {
					let a: Vec<&Rc<Node>> = ary.value().iter().collect();
					Box::new(DepthFirstIterator::from(a))
				}
				NodeValue::Object(obj) => {
					let a: Vec<&Rc<Node>> = obj.value().iter().collect();
					Box::new(DepthFirstIterator::from(a))
				}
			},

			Direction::Breadth => match self.value() {
				NodeValue::Terminal(_) => Box::new(std::iter::empty()),
				NodeValue::Array(ary) => {
					let a: VecDeque<&Rc<Node>> = ary.value().iter().collect();
					Box::new(BreadthFirstIterator::from(a))
				}
				NodeValue::Object(obj) => {
					let a: VecDeque<&Rc<Node>> = obj.value().iter().collect();
					Box::new(BreadthFirstIterator::from(a))
				}
			},
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn breadth() {
		let expected = vec![
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

		let root = test_helper::gen_sample();
		assert_eq!(root.descendants(Direction::Breadth).count(), expected.len());

		for (a, e) in root.descendants(Direction::Breadth).zip(expected.iter()) {
			assert_eq!(&a.full_qualified_name().text_expression(), e)
		}
	}
	#[test]
	fn depth() {
		let expected = vec![
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

		let root = test_helper::gen_sample();
		assert_eq!(root.descendants(Direction::Depth).count(), expected.len());

		for (a, e) in root.descendants(Direction::Depth).zip(expected.iter()) {
			assert_eq!(&a.full_qualified_name().text_expression(), e)
		}
	}

	#[test]
	fn arr_children() {
		let root = test_helper::array();

		for (idx, act) in root.children().enumerate() {
			act.identity().assert_index(idx);
		}

		assert_eq!(
			root.children().count(),
			root.value().extract_array().value().len()
		);
	}

	#[test]
	fn obj_children() {
		let root = test_helper::obj();

		for (idx, elem) in root.children().enumerate() {
			elem.identity().assert_key(&idx.to_string());
		}

		assert_eq!(
			root.children().count(),
			root.value().extract_object().value().len()
		);
	}

	#[test]
	fn terminal_expression() {
		let root = Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string())));
		root.set_identity(Identity::Root);
		let mut children = root.children();

		for _ in 0..10 {
			assert!(children.next().is_none());
		}

		assert_eq!(root.children().count(), 0);
	}

	#[test]
	fn complex_children() {
		let root = test_helper::gen_sample();

		assert_eq!(root.children().count(), 1);
	}

	#[test]
	fn new() {
		let v = NodeValue::Terminal(TerminalNode::String("hello world".to_string()));
		let fixture = Node::new(v);

		fixture
			.value
			.extract_terminal()
			.assert_string("hello world");

		fixture
			.value()
			.extract_terminal()
			.assert_string("hello world");

		fixture.identity().assert_undefined();
	}

	#[test]
	fn set_parent() {
		let fixture = NodeValue::Terminal(TerminalNode::Integer("42".to_string()));
		let mut fixture = vec![Node::new(fixture)];
		for (idx, elem) in fixture.iter_mut().enumerate() {
			elem.set_identity(Identity::from(idx))
		}

		let fixture = NonTerminalNode::new(fixture);

		let fixture = NodeValue::Array(fixture);
		let fixture = Node::new(fixture);

		if let NodeValue::Array(vec) = fixture.value() {
			for elem in vec.value().iter() {
				elem.set_parent(fixture.clone());
			}
		}

		assert!(fixture.parent().is_none());

		if let NodeValue::Array(vec) = fixture.value() {
			for elem in vec.value().iter() {
				let p = elem.parent().unwrap();
				assert!(Rc::ptr_eq(&fixture, &p));
			}
		}
	}

	#[test]
	fn set_identity() {
		let fixture = NodeValue::Terminal(TerminalNode::Integer("42".to_string()));
		let fixture = Node::new(fixture);

		let identity = Identity::from(42);
		fixture.set_identity(identity);

		fixture.identity().assert_index(42);
	}

	#[test]
	fn full_qualified_name() {
		let val = Node::new(NodeValue::Terminal(TerminalNode::Integer("42".to_string())));
		let obj = Node::new(NodeValue::Object(NonTerminalNode::new(vec![val.clone()])));

		for elem in obj.value().extract_object().value().iter() {
			elem.set_parent(obj.clone());
			elem.set_identity(Identity::Key("value".to_string()));
		}

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec![obj.clone()])));

		for elem in arr.value().extract_array().value().iter() {
			elem.set_parent(arr.clone());
			elem.set_identity(Identity::Index(0));
		}

		let root = Node::new(NodeValue::Object(NonTerminalNode::new(vec![arr.clone()])));

		for elem in root.value().extract_object().value().iter() {
			elem.set_identity(Identity::from("arr"));
			elem.set_parent(root.clone());
		}

		root.set_identity(Identity::Root);

		let actual = val.full_qualified_name();
		assert_eq!(4, actual.elements().len());

		let actual = actual.elements();

		assert_eq!(Identity::Root, actual[0]);
		assert_eq!(Identity::Key("arr".to_string()), actual[1]);
		assert_eq!(Identity::Index(0), actual[2]);
		assert_eq!(Identity::Key("value".to_string()), actual[3]);
	}
}

#[cfg(test)]
pub mod test_helper {
	use crate::syntax_node::prelude::*;
	use std::rc::Rc;

	pub const WS: &str = "\u{20}\u{09}\u{0A}\u{0D}";

	pub fn ws() -> String {
		WS.to_string()
	}

	pub fn array() -> Rc<Node> {
		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::Integer("0".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("1".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("2".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("3".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("4".to_string()))),
		];

		let root = Node::new(NodeValue::Array(NonTerminalNode::new(vec)));

		for (idx, elem) in root.value().extract_array().value().iter().enumerate() {
			elem.set_parent(root.clone());
			elem.set_identity(Identity::Index(idx));
		}

		root.set_identity(Identity::Root);
		root
	}

	pub fn obj() -> Rc<Node> {
		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::String("0".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::String("1".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::String("2".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::String("3".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::String("4".to_string()))),
		];

		let root = Node::new(NodeValue::Object(NonTerminalNode::new(vec)));

		for (idx, elem) in root.value().extract_object().value().iter().enumerate() {
			elem.set_parent(root.clone());
			elem.set_identity(Identity::Key(idx.to_string()));
		}

		root.set_identity(Identity::Root);
		root
	}

	pub fn gen_sample() -> Rc<Node> {
		let a = Node::new(NodeValue::Terminal(TerminalNode::Integer("10".to_string())));
		a.set_identity(Identity::Key("1_0".to_string()));

		let b = Node::new(NodeValue::Terminal(TerminalNode::Integer("20".to_string())));
		b.set_identity(Identity::Key("1_1".to_string()));

		let vec = vec![a, b];
		let obj = Node::new(NodeValue::Object(NonTerminalNode::new(vec)));

		for elem in obj.value().extract_object().value().iter() {
			elem.set_parent(obj.clone());
		}

		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::Integer("3".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("4".to_string()))),
		];

		vec[0].set_identity(Identity::Index(0));
		vec[1].set_identity(Identity::Index(1));

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec)));

		for elem in arr.value().extract_array().value() {
			elem.set_parent(arr.clone());
		}

		let vec = vec![
			Node::new(NodeValue::Terminal(TerminalNode::Integer("1".to_string()))),
			Node::new(NodeValue::Terminal(TerminalNode::Integer("2".to_string()))),
			arr,
			obj,
		];

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec)));

		for (idx, elem) in arr.value().extract_array().value().iter().enumerate() {
			elem.set_identity(Identity::Index(idx));
			elem.set_parent(arr.clone());
		}

		let a = Node::new(NodeValue::Terminal(TerminalNode::Null()));
		let b = Node::new(NodeValue::Terminal(TerminalNode::True()));

		a.set_identity(Identity::Key("0_0".to_string()));
		b.set_identity(Identity::Key("1_1".to_string()));

		let obj = Node::new(NodeValue::Object(NonTerminalNode::new(vec![a, b])));

		for elem in obj.value().extract_object().value() {
			elem.set_parent(obj.clone());
		}

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec![obj, arr])));

		for (i, e) in arr.value().extract_array().value().iter().enumerate() {
			e.set_identity(Identity::Index(i));
			e.set_parent(arr.clone());
		}

		let root = Node::new(NodeValue::Object(NonTerminalNode::new(vec![arr])));
		root.set_identity(Identity::Root);
		let piv = &root.value().extract_object().value()[0];
		piv.set_identity(Identity::Key("arr".to_string()));
		piv.set_parent(root.clone());

		root
	}
}
