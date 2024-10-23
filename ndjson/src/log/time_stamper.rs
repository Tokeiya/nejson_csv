use chrono::DateTime;
use chrono::TimeZone;

pub trait TimeStamper<Tz: TimeZone> {
	fn time_stamp() -> DateTime<Tz>;
}
