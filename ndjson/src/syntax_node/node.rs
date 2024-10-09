use super::identity::Identity;
use super::node_value::NodeValue;
use super::prelude::*;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

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
}

#[cfg(test)]
pub mod test_helper {
	pub const WS: &str = "\u{20}\u{09}\u{0A}\u{0D}";

	pub fn ws() -> String {
		WS.to_string()
	}
}

#[cfg(test)]
mod test {
	use super::*;
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
			if let NonTerminalNodeValue::Contents(vec) = vec.value() {
				for elem in vec.iter() {
					elem.set_parent(fixture.clone());
				}
			}
		}

		assert!(fixture.parent().is_none());

		if let NodeValue::Array(vec) = fixture.value() {
			if let NonTerminalNodeValue::Contents(vec) = vec.value() {
				for elem in vec.iter() {
					let p = elem.parent().unwrap();
					assert!(Rc::ptr_eq(&fixture, &p));
				}
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

		for elem in obj
			.value()
			.extract_object()
			.value()
			.extract_contents()
			.iter()
		{
			elem.set_parent(obj.clone());
			elem.set_identity(Identity::Key("value".to_string()));
		}

		let arr = Node::new(NodeValue::Array(NonTerminalNode::new(vec![obj.clone()])));

		for elem in arr
			.value()
			.extract_array()
			.value()
			.extract_contents()
			.iter()
		{
			elem.set_parent(arr.clone());
			elem.set_identity(Identity::Index(0));
		}

		let root = Node::new(NodeValue::Object(NonTerminalNode::new(vec![arr.clone()])));

		for elem in root
			.value()
			.extract_object()
			.value()
			.extract_contents()
			.iter()
		{
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
