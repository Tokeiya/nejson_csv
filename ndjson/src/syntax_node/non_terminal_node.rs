use super::node::Node;

pub enum NonTerminalNodeValue{
	Empty(String),
	Array(Vec<Node>)
}

pub struct NonTerminalNode{
	lead:String,
	value:NonTerminalNodeValue,
	trail:String
}