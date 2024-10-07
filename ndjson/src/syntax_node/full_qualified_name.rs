use super::identity::Identity;
use std::slice::Iter;
pub struct FullQualifiedName(Vec<Identity>);

impl FullQualifiedName {
	pub fn elements(&self) -> &[Identity] {
		&self.0
	}

	pub fn iter(&self) -> Iter<Identity> {
		self.0.iter()
	}
}

impl From<Vec<Identity>> for FullQualifiedName {
	fn from(value: Vec<Identity>) -> Self {
		FullQualifiedName(value)
	}
}

impl PartialEq for FullQualifiedName {
	fn eq(&self, other: &Self) -> bool {
		if self.0.len() == other.0.len() {
			for (a, b) in self.0.iter().zip(other.0.iter()) {
				if a != b {
					return false;
				}
			}
		} else {
			return false;
		}
		true
	}
}

impl Eq for FullQualifiedName {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::syntax_node::identity::Identity;
	use crate::test_helper::test_prelude::*;

	fn generate() -> FullQualifiedName {
		let mut vec = Vec::new();
		vec.push(Identity::Root);
		vec.push(Identity::Key("foo".to_string()));
		vec.push(Identity::Key("bar".to_string()));
		vec.push(Identity::Index(42));
		vec.push(Identity::Index(43));

		FullQualifiedName(vec)
	}

	#[test]
	fn from() {
		let mut vec = Vec::new();
		vec.push(Identity::Root);
		vec.push(Identity::Key("foo".to_string()));
		vec.push(Identity::Key("bar".to_string()));
		vec.push(Identity::Index(42));
		vec.push(Identity::Index(43));

		let fixture = FullQualifiedName::from(vec);
		assert_eq!(fixture.0.len(), 5);

		for (a, e) in fixture.0.iter().zip(generate().0.iter()) {
			assert_eq!(a, e);
		}
	}

	#[test]
	fn iter() {
		let fixture = generate();
		assert_eq!(fixture.0.len(), fixture.iter().len());

		let expected: &[Identity] = &fixture.0;

		for (e, a) in expected.into_iter().zip(fixture.iter()) {
			assert_eq!(a as *const Identity, e as *const Identity);
		}
	}

	#[test]
	fn eq() {
		let mut diff = generate();
		diff.0[0] = Identity::Undefined;

		equivalent(generate(), generate(), generate(), diff);
	}

	#[test]
	fn elements() {
		let fixture = generate();
		let expected = &fixture.0;
		assert_eq!(expected.as_ptr(), fixture.elements().as_ptr());
	}
}
