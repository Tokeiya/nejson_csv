use mockall::mock;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
pub enum Sample {
	A,
	B,
	C,
}

pub struct Envelope(Sample);

pub trait Foo {
	fn get(&self) -> &Envelope;
}

mock! {
	Hoge{}
	impl Foo for Hoge {
		fn get(&self) -> &Envelope;
	}
}
fn main() {
	let mut mock = MockHoge::new();
}
