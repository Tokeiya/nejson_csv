use crate::log::categories::Categories;
use chrono::{DateTime, Local, TimeZone};
use std::fmt::{Debug, Display};

pub struct LogDatum<Tz: TimeZone> {
	time_stamp: DateTime<Tz>,
	category: Categories,
	message: String,
}

impl<Tz: TimeZone> LogDatum<Tz> {
	pub fn new(category: Categories, message: String) -> Self {
		todo!()
	}

	pub fn time_stamp(&self) -> &DateTime<Tz> {
		todo!()
	}

	pub fn category(&self) -> &Categories {
		todo!()
	}

	pub fn message(&self) -> &String {
		todo!()
	}
}

impl<Tz: TimeZone> Debug for LogDatum<Tz> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		todo!()
	}
}

impl<Tz: TimeZone> Display for LogDatum<Tz> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::super::categories::test_helper;
	use super::super::time_stamper::TimeStamper;
	use super::*;
	use chrono::NaiveDate;
	use std::sync::LazyLock;

	static EXPECTED_TIME_STAMP: LazyLock<DateTime<Local>> = LazyLock::new(|| {
		Local
			.from_local_datetime(
				&NaiveDate::from_ymd_opt(2024, 10, 19)
					.unwrap()
					.and_hms_micro_opt(22, 57, 0, 0)
					.unwrap(),
			)
			.unwrap()
	});
	struct MockTimeStamper;

	impl TimeStamper<Local> for MockTimeStamper {
		fn time_stamp(&self) -> DateTime<Local> {
			EXPECTED_TIME_STAMP.clone()
		}
	}

	#[test]
	fn new() {
		for category in test_helper::CATEGORIES.iter() {
			let datum = LogDatum::new(*category, "test".to_string());
			assert_eq!(datum.time_stamp(), EXPECTED_TIME_STAMP.clone());
			assert_eq!(datum.category(), category);
			assert_eq!(datum.message(), "test");
		}
	}
}
