#![feature(format_args_nl)]

mod cache_search;
mod gen_sample;
mod poc;

use combine::Parser;
use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;
use poc::drop_detect::DropDetector;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
fn main() {
	println!("Call bar");
	bar();
	println!("Done");
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

fn foo() {
	gen_sample();

	let current_dir: PathBuf = env::current_dir().unwrap();
	println!("The current directory is: {:?}", current_dir);

	let mut file = File::open("./artifact/hoge.json").unwrap();
	let mut rdr = BufReader::new(&mut file);

	let mut buff = String::new();
	_ = rdr.read_to_string(&mut buff).unwrap();

	let mut parser = value_parser::<&str>();

	let (root, _) = parser.parse(&buff).unwrap();
	root.set_identity(Identity::Root);
}

fn gen_sample() {
	let mut file = File::create("./artifact/hoge.json").unwrap();
	let mut writer = BufWriter::new(&mut file);

	gen_sample::generate_sample(&mut writer, 10).unwrap();
	writer.flush().unwrap();
	println!("done");
}
