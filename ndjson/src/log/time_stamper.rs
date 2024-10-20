use chrono::DateTime;
use chrono::{Local, TimeZone};

pub trait TimeStamper<Tz: TimeZone> {
	fn time_stamp() -> DateTime<Tz>;
}
