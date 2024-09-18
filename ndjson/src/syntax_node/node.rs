use super::non_terminal_node::NonTerminalNode;
use super::terminal_node::TerminalNode;

pub enum Node{
	Terminal(TerminalNode),
	NonTerminal(NonTerminalNode)
}