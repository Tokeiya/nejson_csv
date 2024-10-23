use super::prelude::*;
use chrono::TimeZone;

pub struct ConsoleLogger;

impl<Tz: TimeZone, Ts: TimeStamper<Tz>> Logger<Tz, Ts> for ConsoleLogger {
	fn write(&mut self, categories: Categories, msg: &str) {
		todo!()
	}

	fn write_log(&mut self, datum: LogDatum<Tz>) {
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
