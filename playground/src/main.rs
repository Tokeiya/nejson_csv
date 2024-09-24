use combine as cmb;
use combine::parser::char as chr;
use combine::{ParseError, Parser};

fn letters<'a>() -> impl Parser<&'a str, Output = String> {
	cmb::many1(chr::letter())
}

fn digits<'a>() -> impl Parser<&'a str, Output = String> {
	cmb::many1(chr::digit())
}

fn ws<'a>() -> impl Parser<&'a str, Output = String> {
	chr::spaces().map(|_| "empty".to_string())
}

fn main() {
	let emp = ws().skip(cmb::not_followed_by::<&str, _>(chr::alpha_num()));
	let val = (ws(), chr::alpha_num()).map(|(_, c)| c.to_string());

	let mut foo = cmb::attempt(emp).or(val);

	let r = foo.parse("");
	println!("{r:?}");

	let r = foo.parse("    ");
	println!("{r:?}");

	let r = foo.parse("1");
	println!("{r:?}");

	let r = foo.parse("          1");
	println!("{r:?}");
}
