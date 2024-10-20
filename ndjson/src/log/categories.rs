use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Categories {
	Error,
	Warning,
	Notify,
	Info,
	Verbose,
}

#[cfg(test)]
mod tests {
	use super::test_helper::CATEGORIES;
	use super::Categories;
	use std::fmt::Debug;
	#[test]
	fn debug() {
		let expected = ["Error", "Warning", "Notify", "Info", "Verbose"];
		for (a, e) in CATEGORIES.iter().zip(expected.iter()) {
			assert_eq!(&format!("{:?}", a), e);
		}
	}

	#[test]
	fn copy() {
		let mut a = Categories::Error;
		let b = Categories::Error;

		a = Categories::Notify;

		assert_eq!(a, Categories::Notify);
		assert_eq!(b, Categories::Error);
	}

	#[test]
	fn clone() {
		let mut a = Categories::Notify;
		let b = a.clone();

		a = Categories::Error;
		assert_eq!(a, Categories::Error);
		assert_eq!(b, Categories::Notify);
	}
}
#[cfg(test)]
pub mod test_helper {
	use super::*;
	pub static CATEGORIES: [Categories; 5] = [
		Categories::Error,
		Categories::Warning,
		Categories::Notify,
		Categories::Info,
		Categories::Verbose,
	];
}
