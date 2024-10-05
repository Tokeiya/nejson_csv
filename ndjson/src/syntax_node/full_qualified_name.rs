use super::identity::Identity;
use std::fmt::{Debug, Display, Formatter};
pub struct FullQualifiedName(Vec<Identity>);

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

	pub fn elements(&self) -> &[Identity] {
		&self.0
	}
}

impl From<Vec<Identity>> for FullQualifiedName {
	fn from(value: Vec<Identity>) -> Self {
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
	fn debug() {
		assert_eq!(format!("{:?}", generate()), "Root::foo::bar::[42][43]");
	}

	#[test]
	fn display() {
		assert_eq!(format!("{}", generate()), "Root::foo::bar::[42][43]");
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
		let slice = act.elements();
		let expected = generate();

		assert_eq!(slice.len(), expected.0.len());

		for (a, e) in slice.iter().zip(expected.0.iter()) {
			assert_eq!(a, e)
		}
	}
}
