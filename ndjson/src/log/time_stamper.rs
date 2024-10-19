use chrono::DateTime;
use chrono::{Local, TimeZone};

pub trait TimeStamper<Tz: TimeZone> {
	fn time_stamp(&self) -> DateTime<Tz>;
}
