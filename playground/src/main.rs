mod use_rc;

use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;
use use_rc::*;
pub struct Integer(pub i32);

fn main() {
	let str = "こんにちは世界".to_string();

	println!("{str}");
}
