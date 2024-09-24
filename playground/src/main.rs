use ndjson::parser::value_parser;
use ndjson::syntax_node::prelude::*;

use combine::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

fn write(root: &Node, buff: &mut String) {
	match root.value() {
		NodeValue::Terminal(t) => match t {
			TerminalNode::True() => buff.push_str("true"),
			TerminalNode::False() => buff.push_str("false"),
			TerminalNode::Float(f) => buff.push_str(f),
			TerminalNode::Integer(i) => buff.push_str(i),
			TerminalNode::String(s) => buff.push_str(&format!(r#""{s}""#)),
			TerminalNode::Null() => buff.push_str("null"),
		},
		NodeValue::Array(arr) => {
			buff.push('[');

			match arr.value() {
				NonTerminalNodeValue::Empty(_) => {}
				NonTerminalNodeValue::Contents(c) => {
					for elem in c {
						write(elem, buff);
						buff.push(',');
					}

					buff.remove(buff.len() - 1);
				}
			}

			buff.push(']');
		}
		NodeValue::Object(obj) => {
			buff.push('{');

			match obj.value() {
				NonTerminalNodeValue::Empty(_) => {}
				NonTerminalNodeValue::Contents(c) => {
					for elem in c {
						let NodeValue::Terminal(k) = elem.key().value() else {
							unreachable!()
						};
						let TerminalNode::String(k) = k else {
							unreachable!()
						};

						buff.push_str(&format!(r#""{k}":"#));
						write(elem.value(), buff);
						buff.push(',');
					}
					buff.remove(buff.len() - 1);
					buff.push('}');
				}
			}
		}
	}
}

fn main() {
	let file = File::open("./artifact/sample.json");
	let mut rdr = BufReader::new(file.unwrap());
	let mut scr = String::new();

	let _ = rdr.read_to_string(&mut scr).unwrap();
	let mut parser = value_parser::<&str>();

	let (n, r) = parser.parse(&scr).unwrap();
	assert_eq!(r, "");

	let mut buff = String::new();
	write(&n, &mut buff);

	let mut file = File::create("./artifact/output.json").unwrap();
	let _ = file.write(buff.as_bytes()).unwrap();
}
