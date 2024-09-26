use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;

use combine::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::str::Chars;

fn main() {
	let a = "0123456";
	println!("{}", &a[..1])
}
