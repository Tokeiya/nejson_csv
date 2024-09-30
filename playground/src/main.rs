mod node;
mod use_box_only;
mod use_rc;

use std::ops::Deref;
use std::rc::{Rc, Weak};
//use use_rc::*;
use node::*;
fn main() {
	// let vec = vec![Node::new(NodeValue::Terminal(42), None)];
	// let root = Box::new(Node::new(NodeValue::NonTerminal(vec), None));
	// 
	// let NodeValue::NonTerminal(v) = root.deref().value() else {
	// 	panic!()
	// };
	// v[0].set_parent(Some(root.deref()));
	// 
	// transfer(root)
}

fn transfer(value: Box<Node>) {}
