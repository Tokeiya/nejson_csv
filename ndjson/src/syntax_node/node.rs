use super::node_value::NodeValue;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
	value: NodeValue,
	parent: RefCell<Weak<Node>>,
	lead: String,
	trail: String,
}

impl Node {
	pub fn new(value: NodeValue, lead: String, trail: String) -> Rc<Self> {
		Rc::new(Self {
			value,
			parent: RefCell::new(Weak::default()),
			lead,
			trail,
		})
	}

	pub fn parent(&self) -> Option<Rc<Node>> {
		self.parent.borrow().upgrade()
	}

	pub fn set_parent(&self, parent: Rc<Node>) {
		self.parent.replace(Rc::downgrade(&parent));
	}

	pub fn value(&self) -> &NodeValue {
		&self.value
	}

	pub fn lead(&self) -> &str {
		&self.lead
	}

	pub fn trail(&self) -> &str {
		&self.trail
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	pub const WS: &str = "\u{20}\u{09}\u{0A}\u{0D}";

	pub fn ws() -> String {
		WS.to_string()
	}

	impl Node {
		pub fn assert_lead(&self, expected: Option<&str>) {
			let expected = if let Some(e) = expected { e } else { WS };

			assert_eq!(self.lead.as_str(), expected);
			assert_eq!(self.lead(), expected);
		}

		pub fn assert_trail(&self, expected: Option<&str>) {
			let expected = if let Some(e) = expected { e } else { WS };

			assert_eq!(self.trail.as_str(), expected);
			assert_eq!(self.trail(), expected);
		}

		pub fn assert_lead_trail(&self, lead: Option<&str>, trail: Option<&str>) {
			self.assert_lead(lead);
			self.assert_trail(trail);
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::test_prelude::*;
	use super::*;
	use crate::syntax_node::prelude::*;
	#[test]
	fn new() {
		let v = NodeValue::Terminal(TerminalNode::String("hello world".to_string()));
		let fixture = Node::new(v, ws(), ws());

		fixture
			.value
			.extract_terminal()
			.assert_string("hello world");
		fixture.assert_trail(None);
		fixture.assert_lead(None);

		fixture
			.value()
			.extract_terminal()
			.assert_string("hello world");
	}

	#[test]
	fn set_parent() {
		let fixture = NodeValue::Terminal(TerminalNode::Integer("42".to_string()));
		let fixture = vec![ArrayElement::new(0, Node::new(fixture, ws(), ws()))];
		let fixture = ArrayNode::new(fixture);

		let fixture = NodeValue::Array(fixture);
		let fixture = Node::new(fixture, ws(), ws());

		if let NodeValue::Array(vec) = fixture.value() {
			if let NonTerminalNodeValue::Contents(vec) = vec.value() {
				for elem in vec.iter() {
					let value = elem.value();
					value.set_parent(fixture.clone());
				}
			}
		}

		assert!(fixture.parent().is_none());

		if let NodeValue::Array(vec) = fixture.value() {
			if let NonTerminalNodeValue::Contents(vec) = vec.value() {
				for elem in vec.iter() {
					let p = elem.value().parent().unwrap();
					assert!(Rc::ptr_eq(&fixture, &p));
				}
			}
		}
	}
}
