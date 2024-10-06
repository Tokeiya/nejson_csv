use super::identity::Identity;
use std::collections::{vec_deque::Iter as VecDequeIter, VecDeque};
pub struct FullQualifiedName(VecDeque<Identity>);

impl FullQualifiedName {
	pub fn elements(&self) -> (&[Identity], &[Identity]) {
		self.0.as_slices()
	}

	pub fn iter(&self) -> VecDequeIter<Identity> {
		self.0.iter()
	}
}

impl From<VecDeque<Identity>> for FullQualifiedName {
	fn from(value: VecDeque<Identity>) -> Self {
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
		let mut vec = VecDeque::new();
		vec.push_back(Identity::Root);
		vec.push_back(Identity::Key("foo".to_string()));
		vec.push_back(Identity::Key("bar".to_string()));
		vec.push_back(Identity::Index(42));
		vec.push_back(Identity::Index(43));

		FullQualifiedName(vec)
	}

	#[test]
	fn iter() {
		let expected = generate();
		let (f, l) = expected.elements();
		let mut vec = Vec::new();

		for elem in f.iter() {
			vec.push(elem);
		}

		for elem in l.iter() {
			vec.push(elem);
		}

		let fixture = generate();
		assert_eq!(fixture.0.len(), fixture.iter().len());

		for (e, a) in vec.into_iter().zip(fixture.iter()) {
			assert_eq!(a, e)
		}
	}

	#[test]
	fn from() {
		let mut vec = VecDeque::new();
		vec.push_back(Identity::Root);
		vec.push_back(Identity::Key("foo".to_string()));
		vec.push_back(Identity::Key("bar".to_string()));
		vec.push_back(Identity::Index(42));
		vec.push_back(Identity::Index(43));

		let fixture = FullQualifiedName::from(vec);
		assert_eq!(fixture.0.len(), 5);

		for (a, e) in fixture.0.iter().zip(generate().0.iter()) {
			assert_eq!(a, e);
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
		let act = generate();
		let (f, l) = act.elements();
		let expected = generate();

		assert_eq!((f.len() + l.len()), expected.0.len());

		let mut vec = Vec::new();

		for elem in f {
			vec.push(elem)
		}

		for elem in l {
			vec.push(elem)
		}

		for (a, e) in vec.into_iter().zip(expected.0.iter()) {
			assert_eq!(a, e)
		}
	}
}
