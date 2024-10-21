use super::prelude::*;
use chrono::TimeZone;

pub trait Logger<Tz: TimeZone, Ts: TimeStamper<Tz>> {
	fn write_log(&mut self, datum: LogDatum<Tz, Ts>);
}
