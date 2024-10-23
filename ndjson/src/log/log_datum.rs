use super::time_stamper::TimeStamper;
use crate::log::categories::Categories;
use chrono::{DateTime, Local, TimeZone};
use std::fmt::{Debug, Display};

pub struct LogDatum<Tz: TimeZone> {
	time_stamp: DateTime<Tz>,
	category: Categories,
	message: String,
}

impl<Tz: TimeZone> LogDatum<Tz> {
	pub fn new(time_stamp: DateTime<Tz>, category: Categories, message: String) -> Self {
		Self {
			time_stamp,
			category,
			message,
		}
	}

	pub fn time_stamp(&self) -> &DateTime<Tz> {
		&self.time_stamp
	}

	pub fn category(&self) -> &Categories {
		&self.category
	}

	pub fn message(&self) -> &str {
		&self.message
	}
}

impl<Tz: TimeZone> Debug for LogDatum<Tz> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LogDatum")
			.field("time_stamp", &self.time_stamp)
			.field("category", &self.category)
			.field("message", &self.message)
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use crate::log::prelude::{Categories, LogDatum};
	use crate::log::test_prelude::*;
	use crate::log::time_stamper::TimeStamper;
	use chrono::Local;
	use std::borrow::{Borrow, BorrowMut};
	use std::ops::Deref;

	#[test]
	fn new() {
		for elem in test_categories::CATEGORIES {
			let expected = Local::now();

			let fixture =
				LogDatum::<Local>::new(expected.clone(), elem.clone(), "test".to_string());

			assert_eq!(fixture.time_stamp(), &expected);
			assert_eq!(fixture.category(), &elem);

			assert_eq!(fixture.message(), "test");
		}
	}

	mod time_stamp {
		include!("../test_helper/mock_time_stamper.rs");
	}

	#[test]
	fn time_stamp() {
		for elem in test_categories::CATEGORIES {
			let now = Local::now();
			let fixture = LogDatum::<Local>::new(now.clone(), elem.clone(), "test".to_string());

			assert_eq!(fixture.time_stamp(), &now);
		}
	}

	mod other {
		include!("../test_helper/mock_time_stamper.rs");
	}

	#[test]
	fn category() {
		for elem in test_categories::CATEGORIES {
			let now = Local::now();
			let fixture = LogDatum::<Local>::new(now.clone(), elem, "test".to_string());

			assert_eq!(fixture.category(), &elem);
		}
	}

	#[test]
	fn message() {
		for elem in test_categories::CATEGORIES {
			let fixture = LogDatum::<Local>::new(Local::now(), elem, format!("test {:?}", elem));

			assert_eq!(fixture.message(), &format!("test {:?}", elem));
		}
	}

	#[test]
	fn debug() {
		for elem in test_categories::CATEGORIES {
			let now = Local::now();
			let fixture = LogDatum::<Local>::new(now.clone(), elem, format!("test {:#?}", elem));
			let expected = format!(
				"LogDatum {{ time_stamp: {:?}, category: {:?}, message: {:#?} }}",
				&now,
				elem,
				format!("test {:#?}", elem)
			);

			assert_eq!(format!("{:?}", fixture), expected);
		}
	}
}
