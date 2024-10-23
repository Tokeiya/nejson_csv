#[cfg(test)]
use super::prelude::*;
#[cfg(test)]
use chrono::TimeZone;

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
