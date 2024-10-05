#[derive(Debug, PartialEq, Eq)]
pub enum Identity {
	Key(String),
	Index(usize),
	Root,
	Undefined,
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

		pub fn assert_undefined(&self) {
			let Identity::Undefined = &self else {
				unreachable!()
			};
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helper::test_prelude::*;
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

	#[test]
	fn eq() {
		let x = Identity::from("key");
		let y = Identity::from("key");
		let z = Identity::from("key");
		let not = Identity::from(20);
		equivalent(x, y, z, not);

		equivalent(
			Identity::from(42),
			Identity::from(42),
			Identity::from(42),
			Identity::from(52),
		);

		equivalent(
			Identity::Root,
			Identity::Root,
			Identity::Root,
			Identity::Key("key".to_string()),
		);
	}
}
