use std::fmt::{Debug, Formatter};

pub enum TerminalNodeType {
	Boolean,
	Number,
	String,
	Null,
}

impl Debug for TerminalNodeType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TerminalNodeType::Boolean => write!(f, "Boolean"),
			TerminalNodeType::Number => write!(f, "Number"),
			TerminalNodeType::String => write!(f, "String"),
			TerminalNodeType::Null => write!(f, "Null"),
		}
	}
}

#[cfg(test)]
mod test {
	use super::test_helper::*;
	use super::*;

	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", TerminalNodeType::Boolean), "Boolean");
		assert_eq!(format!("{:?}", TerminalNodeType::Number), "Number");
		assert_eq!(format!("{:?}", TerminalNodeType::String), "String");
		assert_eq!(format!("{:?}", TerminalNodeType::Null), "Null");
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	pub struct TerminalNodeTypes(Option<usize>);

	impl Iterator for TerminalNodeTypes {
		type Item = TerminalNodeType;
		fn next(&mut self) -> Option<Self::Item> {
			if self.0.is_none() {
				None
			} else {
				match self.0.unwrap() {
					0 => {
						self.0 = Some(1);
						Some(TerminalNodeType::Boolean)
					}
					1 => {
						self.0 = Some(2);
						Some(TerminalNodeType::Number)
					}
					2 => {
						self.0 = Some(3);
						Some(TerminalNodeType::String)
					}
					3 => {
						self.0 = Some(4);
						Some(TerminalNodeType::Null)
					}
					_ => {
						self.0 = None;
						None
					}
				}
			}
		}
	}

	#[cfg(test)]
	impl TerminalNodeType {
		#[cfg(test)]
		pub fn assert_boolean(&self) {
			assert!(matches!(self, TerminalNodeType::Boolean));
		}
		#[cfg(test)]
		pub fn assert_number(&self) {
			assert!(matches!(self, TerminalNodeType::Number));
		}
		#[cfg(test)]
		pub fn assert_string(&self) {
			assert!(matches!(self, TerminalNodeType::String));
		}
		#[cfg(test)]
		pub fn assert_null(&self) {
			assert!(matches!(self, TerminalNodeType::Null));
		}
	}
}
