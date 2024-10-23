use chrono::DateTime;
use chrono::TimeZone;

pub trait TimeStamper {
	type Tz: TimeZone;
	fn time_stamp() -> DateTime<Self::Tz>;
}
