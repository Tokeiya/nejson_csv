use super::node_value::NodeValue;
use super::terminal_node::TerminalNode;

pub struct Node {
	value: NodeValue,
	lead: String,
	trail: String,
}

impl Node {
	pub fn new(value: NodeValue, lead: String, trail: String) -> Self {
		Self { value, lead, trail }
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
	use super::super::prelude::*;
	use super::super::test_prelude::*;
	use super::*;
	#[test]
	fn new() {
		let v = NodeValue::Terminal(TerminalNode::new(
			TerminalNodeType::String,
			"hello world".to_string(),
		));
		let fixture = Node::new(v, ws(), ws());

		assert_eq!(fixture.value.extract_terminal().value(), "hello world");
		fixture.assert_trail(None);
		fixture.assert_lead(None);

		assert_eq!(fixture.value().extract_terminal().value(), "hello world");
	}
}
