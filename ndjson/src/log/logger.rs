use super::prelude::*;
use chrono::TimeZone;

pub trait Logger<Tz: TimeZone, Ts: TimeStamper<Tz>> {
	fn write_log(&mut self, datum: LogDatum<Tz>);
	fn write(&mut self, categories: Categories, msg: &str) {
		todo!()
	}
	fn write_error(&mut self, message: &str) {
		todo!()
	}
	fn write_warning(&mut self, message: &str) {
		todo!()
	}
	fn write_info(&mut self, message: &str) {
		todo!()
	}
	fn write_notice(&mut self, message: &str) {
		todo!()
	}
	fn write_verbose(&mut self, message: &str) {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::super::prelude::*;
	use super::super::test_prelude::*;
	use super::*;
	use chrono::{DateTime, Local};
	use mockall::mock;
	use std::cell::Cell;
	use std::ops::{Deref, DerefMut};
	use std::rc::Rc;
	use std::sync::{Arc, LazyLock, Mutex};

	mod mock_time_stamper {
		include!("../test_helper/mock_time_stamper.rs");
	}

	mock! {
		Logger{}
		impl Logger<Local, mock_time_stamper::MockTimeStamper> for Logger {
			fn write_log(&mut self, datum: LogDatum<Local>);
		}
	}

	fn assert_datum<Tz: TimeZone>(
		actual: LogDatum<Tz>,
		expected_ts: &DateTime<Tz>,
		expected_category: Categories,
		expected_msg: &str,
	) {
		assert_eq!(actual.time_stamp(), expected_ts);
		assert_eq!(*actual.category(), expected_category);
		assert_eq!(actual.message(), expected_msg);
	}

	#[test]
	fn write() {
		let expected_category = Arc::new(Mutex::new(Categories::Error));
		let bind = expected_category.clone();

		let mut logger = MockLogger::new();

		logger.expect_write_log().times(5).returning(move |act| {
			assert_datum(
				act,
				&mock_time_stamper::EXPECTED,
				*bind.lock().unwrap().deref(),
				"TEST",
			)
		});

		for cat in test_categories::CATEGORIES.iter() {
			*(expected_category.lock().unwrap().deref_mut()) = cat.clone();

			let datum = LogDatum::new(
				mock_time_stamper::EXPECTED.clone(),
				cat.clone(),
				"TEST".to_string(),
			);
			logger.write_log(datum);
		}
	}

	#[test]
	fn write_error() {}

	#[test]
	fn write_warning() {
		todo!();
	}

	#[test]
	fn write_info() {
		todo!();
	}

	#[test]
	fn write_notice() {
		todo!();
	}

	fn write_verbose() {
		todo!()
	}
}
