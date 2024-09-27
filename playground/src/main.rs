use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;

use combine::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::str::Chars;

fn main() {
	let str = "hello";
	let s = &str[0..1];
	println!("{s}")
}
