use ndjson::parser::value_parser;

use combine::Parser;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
	let file = File::open("./artifact/sample.json").unwrap();
	let mut rdr = BufReader::new(file);

	let mut text = String::new();
	let _ = rdr.read_to_string(&mut text).unwrap();

	let mut parser = value_parser::<&str>();

	let (_, r) = parser.parse(&text).unwrap();
	assert_eq!("", r);
}
