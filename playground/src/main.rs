mod node;

use node::*;
use std::cell::RefCell;

fn main() {
	let tn = NodeValue::terminal_new(42);
	let tn = Node::new(tn, None);

	let root = NodeValue::nonterminal_new(vec![tn]);
	let mut root = Node::new(root, None);

	if let NodeValue::NonTerminal(v) = root.value() {
		v[0].set_parent(Some(&root));
	}
}
