use super::number::number;
use super::{array::array, boolean::boolean, null::null, object::object, string::string};
use crate::syntax_node::prelude::*;
use combine as cmb;
use combine::{choice, parser, Parser, Stream};
use std::rc::Rc;
pub fn ws<I: Stream<Token = char>>() -> impl Parser<I, Output = String> {
	let space = cmb::satisfy::<I, _>(|c| match c {
		'\u{20}' => true,
		'\u{09}' => true,
		'\u{0A}' => true,
		'\u{0D}' => true,
		_ => false,
	});

	cmb::many::<String, I, _>(space)
}

fn value_<I: Stream<Token = char>>() -> impl Parser<I, Output = Rc<Node>> {
	let v = choice!(boolean(), null(), string(), number(), array(), object());
	(ws(), v, ws()).map(|(_, v, _)| {
		let root = Node::new(v);

		if let NodeValue::Array(arr) = root.value() {
			for elem in arr.value().iter() {
				elem.set_parent(root.clone());
			}
		} else if let NodeValue::Object(obj) = root.value() {
			for elem in obj.value().iter() {
				elem.set_parent(root.clone());
			}
		}

		root
	})
}

parser! {
	pub fn value[I]()(I)->Rc<Node>
	where [I:Stream<Token=char>]{
		value_()
	}
}
#[cfg(test)]
mod test {
	use super::*;
	use crate::syntax_node::test_prelude::*;
	use std::borrow::Borrow;
	use std::ptr::eq;

	fn add_ws(s: &str) -> String {
		format!("{WS}{s}{WS}")
	}
	#[test]
	fn string() {
		let str = add_ws(r#""rust""#);
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_string("rust");
	}

	#[test]
	fn integer() {
		let str = add_ws("42");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_integer("42");
	}

	#[test]
	fn float() {
		let str = add_ws("42.195");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_float("42.195");

		let str = add_ws("42.1955e-1");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_float("42.1955e-1");
	}

	#[test]
	fn boolean() {
		let str = add_ws("true");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_true();

		let str = add_ws("false");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_false();
	}

	#[test]
	fn null() {
		let str = add_ws("null");
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.value().extract_terminal().assert_null();
	}

	#[test]
	fn empty_array() {
		let mut parser = super::value::<&str>();
		let (v, r) = parser.parse("[   ]").unwrap();
		assert_eq!(r, "");
		assert_eq!(v.value().extract_array().value().len(), 0);
	}
	#[test]
	fn array() {
		let str = add_ws("[1,  2,3]");
		let mut parser = super::value::<&str>(); //::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		assert_eq!(rem, "");
		a.identity().assert_undefined();
		let arr = a.value().extract_array().value();

		assert_eq!(arr.len(), 3);
		arr[0].value().extract_terminal().assert_integer("1");

		arr[0].identity().assert_index(0);

		arr[1].value().extract_terminal().assert_integer("2");

		arr[1].identity().assert_index(1);

		arr[2].value().extract_terminal().assert_integer("3");

		arr[2].identity().assert_index(2);
	}

	#[test]
	fn object() {
		let str = add_ws(r#"{"a": 1, "b": 2, "c": 3}"#);
		let mut parser = super::value::<&str>();
		let (a, rem) = parser.parse(&str).unwrap();
		a.identity().assert_undefined();
		assert_eq!(rem, "");
		let obj = a.value().extract_object().value();

		assert_eq!(obj.len(), 3);
		obj[0].identity().assert_key("a");
		obj[0].value().extract_terminal().assert_integer("1");

		obj[1].identity().assert_key("b");
		obj[1].value().extract_terminal().assert_integer("2");

		obj[2].identity().assert_key("c");
		obj[2].value().extract_terminal().assert_integer("3");
	}

	#[test]
	fn complex() {
		let mut parser = super::value::<&str>();
		let (a, r) = parser
			.parse(r#"{"obj"  :{   "o":10}      ,"arr":[1,2,3]}"#)
			.unwrap();
		assert_eq!(r, "");

		let obj = a.value().extract_object().value();

		assert_eq!(obj.len(), 2);

		assert_eq!(
			obj[0].parent().unwrap().borrow() as *const Node,
			a.borrow() as *const Node
		);

		assert_eq!(
			obj[1].parent().unwrap().borrow() as *const Node,
			a.borrow() as *const Node
		);

		let piv = &obj[0];

		let inner = piv.value().extract_object().value();
		assert_eq!(inner.len(), 1);
		assert_eq!(
			piv.borrow() as *const Node,
			inner[0].parent().unwrap().borrow() as *const Node
		);

		inner[0].identity().assert_key("o");
		inner[0].value().extract_terminal().assert_integer("10");
	}

	#[test]
	fn ws() {
		use crate::syntax_node::test_prelude::ws;

		for expected in WS.chars().map(|c| c.to_string()) {
			let mut parser = super::ws::<&str>();
			let (a, r) = parser.parse(&expected).unwrap();
			assert_eq!(r, "");
			assert_eq!(a, expected);
		}

		let mut parser = super::ws::<&str>();
		let (a, r) = parser.parse(WS).unwrap();
		assert_eq!(r, "");
		assert_eq!(a, ws());

		let (a, r) = parser.parse("").unwrap();
		assert_eq!("", a);
		assert_eq!("", r)
	}

	#[test]
	fn parent() {
		let mut parser = value::<&str>();
		let (root, _) = parser.parse("[1,2]").unwrap();

		if let NodeValue::Array(arr) = root.value() {
			for elem in arr.value().iter() {
				let c = Rc::as_ptr(&elem.parent().unwrap());
				let r = Rc::as_ptr(&root);
				assert!(eq(r, c));
			}
		}
	}
}
