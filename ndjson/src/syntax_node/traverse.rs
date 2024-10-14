use super::prelude::*;
use std::rc::Rc;

pub enum Direction {
	Breadth,
	Depth,
}

pub trait Traverse {
	fn children(&self) -> Box<dyn Iterator<Item = &Rc<Node>>>;
	fn ancestors(&self, direction: Direction) -> Box<dyn Iterator<Item = &Rc<Node>>>;
}

impl Traverse for Rc<Node> {
	fn children(&self) -> Box<dyn Iterator<Item = &Rc<Node>>> {
		todo!()
	}

	fn ancestors(&self, direction: Direction) -> Box<dyn Iterator<Item = &Rc<Node>>> {
		todo!()
	}
}
