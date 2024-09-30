use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum Value {
	Terminal(i32),
	NonTerminal(Vec<Rc<Node>>),
}

pub struct Node {
	value: Value,
	parent: RefCell<Weak<Node>>,
}

impl Node {
	pub fn new_terminal(value: i32) -> Rc<Self> {
		Rc::new(Self {
			value: Value::Terminal(value),
			parent: RefCell::new(Weak::default()),
		})
	}

	pub fn new_nonterminal(children: Vec<Rc<Node>>) -> Rc<Self> {
		let ret = Rc::new(Self {
			value: Value::NonTerminal(children),
			parent: RefCell::new(Weak::default()),
		});

		let Value::NonTerminal(vec) = ret.value() else {
			unreachable!()
		};

		for elem in vec.iter() {
			elem.set_parent(ret.clone());
		}

		ret
	}

	fn set_parent(&self, parent: Rc<Node>) {
		let parent = Rc::downgrade(&parent);
		self.parent.replace(parent);
	}

	pub fn parent(&self) -> Option<Rc<Node>> {
		self.parent.borrow().upgrade()
	}

	pub fn value(&self) -> &Value {
		&self.value
	}
}
