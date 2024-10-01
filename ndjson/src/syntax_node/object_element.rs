use super::object_identity::ObjectIdentity;
use super::prelude::*;
use std::rc::Rc;

pub struct ObjectElement {
	key: ObjectIdentity,
	value: Rc<Node>,
}

impl ObjectElement {
	pub fn new(key: Rc<Node>, value: Rc<Node>) -> Self {
		let NodeValue::Terminal(key) = key.value() else {
			unreachable!()
		};

		let TerminalNode::String(key) = key else {
			unreachable!()
		};

		let key = ObjectIdentity::from(key.as_str());

		Self { key, value }
	}
	pub fn key(&self) -> &ObjectIdentity {
		&self.key
	}
	pub fn value(&self) -> &Node {
		&self.value
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl ObjectElement {
		pub fn assert_key(&self, value: &str) {
			(&self.key).assert_raw(value);
			(&self.key).assert_escaped(value);

			let key = self.key();
			key.assert_raw(value);
			key.assert_escaped(value);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn new() {
		let key = Node::new(NodeValue::Terminal(TerminalNode::String("key".to_string())));
		let value = Node::new(NodeValue::Terminal(TerminalNode::String(
			"value".to_string(),
		)));

		let object_element = ObjectElement::new(key, value);
		object_element.assert_key("key");
		object_element
			.value
			.value()
			.extract_terminal()
			.assert_string("value");

		object_element
			.value()
			.value()
			.extract_terminal()
			.assert_string("value");
	}
}
