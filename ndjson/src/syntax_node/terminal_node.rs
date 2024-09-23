pub enum TerminalNode {
	True(),
	False(),
	Float(String),
	Integer(String),
	String(String),
	Null(),
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl TerminalNode {
		pub fn assert_string(&self, expected: &str) {
			let Self::String(s) = self else {
				unreachable!()
			};
			assert_eq!(s, expected)
		}

		pub fn assert_true(&self) {
			let Self::True() = self else { unreachable!() };
		}

		pub fn assert_false(&self) {
			let Self::False() = self else { unreachable!() };
		}

		pub fn assert_float(&self, expected: &str) {
			let Self::Float(a) = self else { unreachable!() };
			assert_eq!(a, expected);
		}

		pub fn assert_integer(&self, expected: &str) {
			let Self::Integer(a) = self else {
				unreachable!()
			};
			assert_eq!(a, expected)
		}

		pub fn assert_null(&self) {
			let Self::Null() = self else { unreachable!() };
		}
	}
}
