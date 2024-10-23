use super::prelude::*;
use chrono::TimeZone;
use combine::parser::combinator::Lazy;

pub struct ConsoleLogger;

#[cfg(test)]

impl<Tz: TimeZone, Ts: TimeStamper<Tz>> Logger<Tz, Ts> for ConsoleLogger {
	fn write_log(&mut self, datum: LogDatum<Tz>) {
		println!(
			"{:?} {:?} {}",
			datum.time_stamp(),
			datum.category(),
			datum.message()
		);
	}
}
