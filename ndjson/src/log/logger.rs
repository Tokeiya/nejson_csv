use super::prelude::*;
use chrono::TimeZone;

pub trait Logger {
	type Tz: TimeZone;
	type Ts: TimeStamper<Tz = Self::Tz>;
	fn write_log(&mut self, datum: LogDatum<Self::Tz>);
	fn write(&mut self, categories: Categories, msg: &str) {
		let datum = LogDatum::new(Self::Ts::time_stamp(), categories, msg.to_string());
		self.write_log(datum);
	}
	fn write_error(&mut self, message: &str) {
		let datum = LogDatum::new(
			Self::Ts::time_stamp(),
			Categories::Error,
			message.to_string(),
		);
		self.write_log(datum);
	}
	fn write_warning(&mut self, message: &str) {
		let datum = LogDatum::new(
			Self::Ts::time_stamp(),
			Categories::Warning,
			message.to_string(),
		);
		self.write_log(datum);
	}
	fn write_info(&mut self, message: &str) {
		let datum = LogDatum::new(
			Self::Ts::time_stamp(),
			Categories::Info,
			message.to_string(),
		);
		self.write_log(datum);
	}
	fn write_notify(&mut self, message: &str) {
		let datum = LogDatum::new(
			Self::Ts::time_stamp(),
			Categories::Notify,
			message.to_string(),
		);
		self.write_log(datum);
	}
	fn write_verbose(&mut self, message: &str) {
		let datum = LogDatum::new(
			Self::Ts::time_stamp(),
			Categories::Verbose,
			message.to_string(),
		);
		self.write_log(datum);
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	use chrono::Local;

	pub mod time_stamper {
		include!("../test_helper/mock_time_stamper.rs");
	}

	pub struct MockLogger(Vec<LogDatum<Local>>);

	impl MockLogger {
		pub fn new() -> Self {
			Self(Vec::new())
		}

		pub fn get_log(&self) -> &[LogDatum<Local>] {
			&self.0
		}
	}

	impl Logger for MockLogger {
		type Tz = Local;
		type Ts = time_stamper::MockTimeStamper;

		fn write_log(&mut self, datum: LogDatum<Local>) {
			self.0.push(datum);
		}
	}
}
#[cfg(test)]
mod test {
	use super::*;
	use chrono::{DateTime, Local};
	use std::ops::Deref;
	use test_helper::MockLogger;

	fn assert_log_datum(
		datum: &LogDatum<Local>,
		expected_time_stamp: &DateTime<Local>,
		expected_category: Categories,
		expected_message: &str,
	) {
		assert_eq!(datum.time_stamp(), expected_time_stamp);
		assert_eq!(datum.category(), &expected_category);
		assert_eq!(datum.message(), expected_message);
	}

	#[test]
	fn write() {
		let mut mock = MockLogger::new();

		mock.write(Categories::Error, "error");
		mock.write(Categories::Warning, "warning");
		mock.write(Categories::Info, "info");
		mock.write(Categories::Notify, "notify");
		mock.write(Categories::Verbose, "verbose");

		assert_eq!(mock.get_log().len(), 5);

		let act = mock.get_log();

		assert_log_datum(
			&act[0],
			&test_helper::time_stamper::EXPECTED.deref(),
			Categories::Error,
			"error",
		);

		assert_log_datum(
			&act[1],
			&test_helper::time_stamper::EXPECTED.deref(),
			Categories::Warning,
			"warning",
		);

		assert_log_datum(
			&act[2],
			&test_helper::time_stamper::EXPECTED.deref(),
			Categories::Info,
			"info",
		);

		assert_log_datum(
			&act[3],
			&test_helper::time_stamper::EXPECTED.deref(),
			Categories::Notify,
			"notify",
		);

		assert_log_datum(
			&act[4],
			&test_helper::time_stamper::EXPECTED.deref(),
			Categories::Verbose,
			"verbose",
		)
	}
	#[test]
	fn write_error() {
		let mut mock = MockLogger::new();

		for i in 0..10usize {
			mock.write_error(format!("{i}").as_str());
		}

		assert_eq!(mock.get_log().len(), 10);

		for (idx, act) in mock.get_log().iter().enumerate() {
			assert_log_datum(
				act,
				&test_helper::time_stamper::EXPECTED.deref(),
				Categories::Error,
				&format!("{idx}"),
			);
		}
	}

	#[test]
	fn write_warning() {
		let mut mock = MockLogger::new();

		for i in 0..10usize {
			mock.write_warning(format!("{i}").as_str());
		}

		assert_eq!(mock.get_log().len(), 10);

		for (idx, act) in mock.get_log().iter().enumerate() {
			assert_log_datum(
				act,
				&test_helper::time_stamper::EXPECTED.deref(),
				Categories::Warning,
				&format!("{idx}"),
			)
		}
	}

	#[test]
	fn write_info() {
		let mut mock = MockLogger::new();

		for i in 0..10usize {
			mock.write_info(format!("{i}").as_str());
		}

		for (idx, act) in mock.get_log().iter().enumerate() {
			assert_log_datum(
				act,
				&test_helper::time_stamper::EXPECTED.deref(),
				Categories::Info,
				&format!("{idx}"),
			)
		}
	}

	#[test]
	fn write_notify() {
		let mut mock = MockLogger::new();

		for i in 0..10usize {
			mock.write_notify(format!("{i}").as_str());
		}

		for (idx, act) in mock.get_log().iter().enumerate() {
			assert_log_datum(
				act,
				&test_helper::time_stamper::EXPECTED.deref(),
				Categories::Notify,
				&format!("{idx}"),
			)
		}
	}

	#[test]
	fn write_verbose() {
		let mut mock = MockLogger::new();

		for i in 0..10usize {
			mock.write_verbose(format!("{i}").as_str());
		}

		for (idx, act) in mock.get_log().iter().enumerate() {
			assert_log_datum(
				act,
				&test_helper::time_stamper::EXPECTED.deref(),
				Categories::Verbose,
				&format!("{idx}"),
			)
		}
	}
}
