#![feature(format_args_nl)]
#![allow(dead_code)]
mod gen_sample;
mod poc;

use combine::Parser;
use std::collections::VecDeque;
use std::io::Read;
use std::slice::Iter;

pub struct Integer(i32);

pub struct Foo<'a> {
	vec: Vec<Integer>,
	queue: VecDeque<Iter<'a, Integer>>,
}

impl<'a> Foo<'a> {
	pub fn new(vec: Vec<Integer>) -> Foo<'a> {
		let queue: VecDeque<_> = VecDeque::new(); // イテレータを集める
		Foo { vec, queue }
	}

	pub fn add_iter(&'a mut self) {
		self.queue.push_back(self.vec.iter());
	}
}

fn main() {}
