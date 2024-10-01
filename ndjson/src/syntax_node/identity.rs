pub enum Identity {
	Key(String),
	Index(usize),
	Root,
}

impl From<&str> for Identity {
	fn from(value: &str) -> Self {
		Identity::Key(value.to_string())
	}
}

impl From<usize> for Identity {
	fn from(value: usize) -> Self {
		Identity::Index(value)
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl Identity {
		pub fn assert_index(&self, expected: usize) {
			let Identity::Index(act) = &self else {
				unreachable!()
			};
			assert_eq!(act, &expected)
		}

		pub fn assert_root(&self) {
			let Identity::Root = &self else {
				unreachable!()
			};
		}

		pub fn assert_key(&self, key: &str) {
			let Identity::Key(act) = &self else {
				unreachable!()
			};
			assert_eq!(act, key)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn str_from() {
		let fixture = Identity::from("key");
		fixture.assert_key("key");
	}

	#[test]
	fn usize_from() {
		let fixture = Identity::from(42);
		fixture.assert_index(42)
	}
}
