use rand::prelude::*;
use std::cell::Cell;
use std::collections::hash_set::HashSet;
use std::io::{Result as IoResult, Write};

const MAX_ELEMENTS: usize = 10;
const INDENT: usize = 2;

enum RecentNode {
	Array,
	Object,
	Root,
}

enum NodeTypes {
	Array,
	Object,
	Terminal,
}

fn choice_keys(rnd: &mut ThreadRng, size: usize) -> Vec<char> {
	let mut set = HashSet::new();
	let mut keys: Vec<char> = Vec::new();
	let mut idx = 0usize;

	loop {
		let candidate = char::from_u32(rnd.gen_range(0x61..=0x7a)).unwrap();
		if set.insert(candidate) {
			keys.push(candidate);
			idx += 1;
			if idx == size {
				return keys;
			}
		}
	}
}

fn choice_node(rnd: &mut ThreadRng) -> NodeTypes {
	let n = rnd.gen_range(0..=2);
	match n {
		0 => NodeTypes::Array,
		1 => NodeTypes::Object,
		2 => NodeTypes::Terminal,
		_ => panic!("invalid choice"),
	}
}

fn get_indent<'a>(indent: &'a str, recent: &'a RecentNode) -> &'a str {
	match recent {
		RecentNode::Array => indent,
		RecentNode::Object => "",
		RecentNode::Root => "",
	}
}

fn generate_object(
	writer: &mut impl Write,
	rnd: &mut ThreadRng,
	cnt: &Cell<usize>,
	current_depth: usize,
	threshold_depth: usize,
	recent: RecentNode,
	indent_cnt: usize,
) -> IoResult<()> {
	let indent = std::iter::repeat(" ").take(indent_cnt).collect::<String>();
	let str = indent.as_str();

	writer.write_fmt(format_args_nl!(
		"{}{{",
		get_indent(indent.as_str(), &recent)
	))?;
	let size = rnd.gen_range(1..=MAX_ELEMENTS);
	let keys = choice_keys(rnd, size);

	for (idx, key) in keys.iter().enumerate() {
		writer.write_fmt(format_args!(r#" {indent}"{key}":"#))?;
		generate_node(
			writer,
			rnd,
			cnt,
			current_depth + 1,
			threshold_depth,
			RecentNode::Object,
			indent_cnt + INDENT + 1,
		)?;

		if size - 1 > idx {
			writer.write_fmt(format_args_nl!(","))?;
		}
	}

	writer.write_fmt(format_args!("\n{indent}}}"))?;

	Ok(())
}

fn generate_array(
	writer: &mut impl Write,
	rnd: &mut ThreadRng,
	cnt: &Cell<usize>,
	current_depth: usize,
	threshold_depth: usize,
	recent: RecentNode,
	indent_cnt: usize,
) -> IoResult<()> {
	let indent = std::iter::repeat(" ").take(indent_cnt).collect::<String>();

	let str = indent.as_str();

	writer.write_fmt(format_args_nl!("{}[", get_indent(indent.as_str(), &recent)))?;

	let size = rnd.gen_range(1..=MAX_ELEMENTS);

	for idx in 0..size {
		generate_node(
			writer,
			rnd,
			cnt,
			current_depth + 1,
			threshold_depth,
			RecentNode::Array,
			indent_cnt + INDENT,
		)?;

		if size - 1 > idx {
			writer.write_fmt(format_args_nl!(","))?;
		}
	}

	writer.write_fmt(format_args!("\n{}]", indent.as_str()))?;
	Ok(())
}

fn generate_node(
	writer: &mut impl Write,
	rnd: &mut ThreadRng,
	cnt: &Cell<usize>,
	current_depth: usize,
	threshold_depth: usize,
	recent: RecentNode,
	indent_cnt: usize,
) -> IoResult<()> {
	let indent = std::iter::repeat(" ").take(indent_cnt).collect::<String>();

	if current_depth >= threshold_depth {
		writer.write_fmt(format_args!(
			"{}{}",
			get_indent(indent.as_str(), &recent),
			cnt.get()
		))?;
		cnt.set(cnt.get() + 1);
		Ok(())
	} else {
		match choice_node(rnd) {
			NodeTypes::Array => generate_array(
				writer,
				rnd,
				cnt,
				current_depth + 1,
				threshold_depth,
				recent,
				indent_cnt + INDENT,
			),
			NodeTypes::Object => generate_object(
				writer,
				rnd,
				cnt,
				current_depth + 1,
				threshold_depth,
				recent,
				indent_cnt + INDENT,
			),
			NodeTypes::Terminal => {
				writer.write_fmt(format_args!(
					"{}{}",
					get_indent(indent.as_str(), &recent),
					cnt.get()
				))?;
				cnt.set(cnt.get() + 1);
				Ok(())
			}
		}
	}
}

pub fn generate_sample(writer: &mut impl Write, depth: usize) -> IoResult<()> {
	let mut rnd = rand::thread_rng();
	generate_node(
		writer,
		&mut rnd,
		&Cell::new(0),
		0,
		depth,
		RecentNode::Root,
		0,
	)?;

	Ok(())
}
