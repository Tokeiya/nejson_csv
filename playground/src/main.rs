use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;

use combine::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() {
	let file = File::open("./artifact/sample.json").unwrap();
	let mut rdr = BufReader::new(file);

	let mut text = String::new();
	let r = rdr.read_to_string(&mut text).unwrap();

	let mut parser = value_parser::<&str>();

	let (a, r) = parser.parse(&text).unwrap();
	assert_eq!("", r);
}
