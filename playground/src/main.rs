use combine as cmb;
use combine::{satisfy, Parser};
fn main() {
	for x in (1..=9).map(|c| c.to_string()) {
		let mut one_nine = cmb::satisfy::<&str, _>(|c| c >= '1' && c <= '9');
		let a = one_nine.parse(x.as_str()).unwrap();
	}
}
