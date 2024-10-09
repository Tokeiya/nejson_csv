use super::identity::Identity;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
pub struct FullQualifiedName(VecDeque<Identity>);

impl FullQualifiedName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for elem in self.0.iter() {
			match elem {
				Identity::Key(k) => f.write_fmt(format_args!("{k}::"))?,
				Identity::Index(i) => f.write_fmt(format_args!("[{i}]"))?,
				Identity::Root => f.write_str("Root::")?,
				Identity::Undefined => f.write_str("Undefined::")?,
			}
		}

		Ok(())
	}

	pub fn elements(&self) -> (&[Identity], &[Identity]) {
		self.0.as_slices()
	}
}

impl From<VecDeque<Identity>> for FullQualifiedName {
	fn from(value: VecDeque<Identity>) -> Self {
		FullQualifiedName(value)
	}
}

impl Debug for FullQualifiedName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
	}
}

impl Display for FullQualifiedName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.fmt(f)
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
		vec.push_back(Identity::Key("hoge".to_string()));

		FullQualifiedName(vec)
	}

	#[test]
	fn from() {
		let mut vec = VecDeque::new();
		vec.push_back(Identity::Root);
		vec.push_back(Identity::Key("foo".to_string()));
		vec.push_back(Identity::Key("bar".to_string()));
		vec.push_back(Identity::Index(42));
		vec.push_back(Identity::Index(43));
		vec.push_back(Identity::Key("hoge".to_string()));

		let fixture = FullQualifiedName::from(vec);
		assert_eq!(fixture.0.len(), 6);

		for (a, e) in fixture.0.iter().zip(generate().0.iter()) {
			assert_eq!(a, e);
		}
	}
	#[test]
	fn debug() {
		assert_eq!(
			format!("{:?}", generate()),
			"Root::foo::bar::[42][43]hoge::"
		);
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", generate()), "Root::foo::bar::[42][43]hoge::");
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
