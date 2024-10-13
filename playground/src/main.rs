#![feature(format_args_nl)]

mod cache_search;
mod gen_sample;

use combine::Parser;
use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;
use std::cell::Ref;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

fn main() {
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
