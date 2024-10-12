#![feature(format_args_nl)]

mod cache_search;
mod gen_sample;

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
	let mut file = File::create("./artifact/hoge.json").unwrap();
	let mut writer = BufWriter::new(&mut file);

	gen_sample::generate_sample(&mut writer, 10).unwrap();
	writer.flush().unwrap();
	println!("done");
}
