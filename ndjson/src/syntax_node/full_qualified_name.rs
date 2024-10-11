use super::char_continuous_counter::CharContinuousCounter;
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

	pub fn text_expression(&self) -> String {
		let mut colon = CharContinuousCounter::<':'>::new();
		let mut left = CharContinuousCounter::<'['>::new();
		let mut right = CharContinuousCounter::<']'>::new();

		for elem in self.0.iter().filter(|x| matches!(x, Identity::Key(_))) {
			let Identity::Key(k) = elem else {
				unreachable!()
			};

			for c in k.chars() {
				colon.input(c);
				left.input(c);
				right.input(c);
			}
		}

		let coron = std::cmp::max(colon.max() + 1, 2);
		let bracket = std::cmp::max(left.max(), right.max()) + 1;

		let coron = ":".repeat(coron);
		let left = "[".repeat(bracket);
		let right = "]".repeat(bracket);

		let mut buff = String::new();

		for elem in self.0[..self.0.len() - 1].iter() {
			match elem {
				Identity::Key(key) => {
					buff.push_str(key);
					buff.push_str(&coron);
				}
				Identity::Index(idx) => {
					buff.push_str(&left);
					buff.push_str(&idx.to_string());
					buff.push_str(&right);
				}
				Identity::Root => {
					buff.push_str("Root");
					buff.push_str(&coron);
				}
				Identity::Undefined => {
					buff.push_str("Undefined");
					buff.push_str(&coron);
				}
			}
		}

		match &self.0[self.0.len() - 1] {
			Identity::Key(key) => buff.push_str(&key),
			Identity::Index(idx) => {
				buff.push_str(&left);
				buff.push_str(&idx.to_string());
				buff.push_str(&right);
			}
			Identity::Root => buff.push_str("Root"),
			Identity::Undefined => buff.push_str("Undefined"),
		}

		buff
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

	#[test]
	fn text_expression() {
		let vec = vec![
			Identity::Key("foo".to_string()),
			Identity::Index(42),
			Identity::Key("bar".to_string()),
		];
		let fixture = FullQualifiedName::from(vec);
		assert_eq!(fixture.text_expression(), "foo::[42]bar");

		let vec = vec![Identity::Key("fo::o".to_string()), Identity::Index(42)];
		let fixture = FullQualifiedName::from(vec);

		assert_eq!(fixture.text_expression(), "fo::o:::[42]");

		let vec = vec![Identity::Key("fo::[o]".to_string()), Identity::Index(42)];
		let fixture = FullQualifiedName::from(vec);
		assert_eq!(fixture.text_expression(), "fo::[o]:::[[42]]");
	}
}
