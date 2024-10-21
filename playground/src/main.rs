#[derive(Debug)]
struct Foo {
	key: i32,
	value: String,
}

fn main() {
	let a = Foo {
		key: 1,
		value: "test".to_string(),
	};
	println!("{:?}", a);
}
