mod use_rc;

pub struct Integer(pub i32);

fn main() {
	let str = "こんにちは世界".to_string();

	let a = str.contains("foo");
	println!("{str}");
}
