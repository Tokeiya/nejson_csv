use super::prelude::*;
use chrono::TimeZone;
use std::marker::PhantomData;

pub struct ConsoleLogger<Tz, Ts>(PhantomData<(Tz, Ts)>);

impl<Tz: TimeZone, Ts: TimeStamper<Tz = Tz>> Logger for ConsoleLogger<Tz, Ts> {
	type Tz = Tz;
	type Ts = Ts;

	fn write_log(&mut self, datum: LogDatum<Tz>) {
		println!(
			"{:?} {:?} {}",
			datum.time_stamp(),
			datum.category(),
			datum.message()
		);
	}
}
