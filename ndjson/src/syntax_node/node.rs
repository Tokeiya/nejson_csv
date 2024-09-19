use super::non_terminal_node::NonTerminalNode;
use super::object_element::ObjectElement;
use super::terminal_node::TerminalNode;
pub enum Node {
	Terminal(TerminalNode),
	Non,
}

#[cfg(test)]
pub mod test_helper {
	use std::sync::LazyLock;

	pub static WS: LazyLock<&'static str> = LazyLock::new(|| "\u{20}\u{09}\u{0A}\u{0D}");

	pub fn ws() -> String {
		WS.to_string()
	}
}
