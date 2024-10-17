use chrono::{DateTime, TimeZone};

pub trait TimeStamper<Tz: TimeZone> {
	fn time_stamp() -> DateTime<Tz>;
}
