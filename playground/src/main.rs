#![feature(format_args_nl)]

mod cache_search;
mod gen_sample;
mod poc;

use combine::Parser;
use poc::drop_detect::DropDetector;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::rc::Rc;
use std::slice::Iter;

pub struct Identity(i32);

fn main() {
	let mut vec = Rc::new(Vec::<i32>::new());
	let mut iter: Iter<i32> = vec.iter();
}

fn bar() {
	println!("Enter bar");
	let mut vec = VecDeque::new();
	for _ in 0..10 {
		vec.push_front(DropDetector::new())
	}

	for elem in vec.iter() {
		println!("{:?}", elem.identity());
	}

	println!("Exit bar");
}

fn gen_sample() {
	let mut file = File::create("./artifact/hoge.json").unwrap();
	let mut writer = BufWriter::new(&mut file);

	gen_sample::generate_sample(&mut writer, 10).unwrap();
	writer.flush().unwrap();
	println!("done");
}
