use combine::{self as cmb, parser::char as chr, Parser, Stream};

pub enum Letter {
	Char(char),
	String(String),
}

fn unescaped<'a>() -> impl Parser<&'a str, Output = Letter> {
	cmb::satisfy::<&'a str, _>(|c| {
		(c >= '\u{20}' && c <= '\u{21}')
			|| (c >= '\u{23}' && c <= '\u{5B}')
			|| (c >= '\u{5D}' && c <= '\u{10FFFF}')
	})
	.map(|c| Letter::Char(c))
}
