mod char_continuous_counter;
mod full_qualified_name;
mod identity;
mod node;
mod node_value;
mod non_terminal_node;
mod object_identity;
mod terminal_node;

pub mod prelude {
	pub use super::char_continuous_counter::CharContinuousCounter;
	pub use super::full_qualified_name::FullQualifiedName;
	pub use super::identity::Identity;
	pub use super::node::Node;
	pub use super::node_value::NodeValue;
	pub use super::non_terminal_node::{NonTerminalNode, NonTerminalNodeValue};
	pub use super::object_identity::ObjectIdentity;
	pub use super::terminal_node::TerminalNode;
}

#[cfg(test)]
pub mod test_prelude {
	pub use super::terminal_node::test_helper as terminal_node_helper;

	pub use super::identity::test_helper as identity_helper;
	pub use super::node::test_helper::{self as node_helper, ws, WS};
	pub use super::node_value::test_helper as node_value_helper;
	pub use super::non_terminal_node::test_helper as non_terminal_node_helper;
}
