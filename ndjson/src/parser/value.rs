use crate::syntax_node::prelude::Node;
use combine as cmb;
use combine::parser;
use combine::parser::char as chr;
use combine::{Parser, Stream};

pub fn value<I: Stream<Token = char>>() -> impl Parser<I, Output = Node> {
	chr::char('a').map(|_| todo!())
}
