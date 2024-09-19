use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalNodeType {
	Boolean,
	Number,
	String,
	Null,
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

	#[test]
	fn copy() {
		let fixture = TerminalNodeType::String;
		let copied = fixture;

		fixture.assert_string();
		copied.assert_string();
	}

	#[test]
	fn clone() {
		let fixture = TerminalNodeType::Boolean;
		let cloned = fixture.clone();

		fixture.assert_boolean();
		cloned.assert_boolean();
	}

	#[test]
	fn eq() {
		for elem in TerminalNodeTypes::new() {
			let foo = elem;
			assert_eq!(foo, elem);
		}

		assert_ne!(TerminalNodeType::Boolean, TerminalNodeType::String);
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	pub struct TerminalNodeTypes(Option<usize>);

	impl TerminalNodeTypes {
		pub fn new() -> Self {
			TerminalNodeTypes(Some(0))
		}
	}

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
