mod alt_node;
mod node;

use alt_node::*;

fn main() {
	let root = Node::new_nonterminal(None);

	let child = Node::new_terminal(42, Some(&root));
	root.add_child(child);

	foo(root);
}

fn foo<'a>(root: Node<'a>) {}
