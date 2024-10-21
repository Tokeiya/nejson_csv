use super::time_stamper::TimeStamper;
use chrono::{DateTime, Local, TimeZone};

pub struct LocalTimeStamper;

impl TimeStamper<Local> for LocalTimeStamper {
	fn time_stamp() -> DateTime<Local> {
		Local::now()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use chrono::Duration;

	#[test]
	fn time_stamp() {
		let tolerance = Duration::milliseconds(100);
		let single_tolerance = Duration::milliseconds(250);
		let mut accum = Duration::zero();

		for _ in 0..100 {
			let now = Local::now();
			let stamp = LocalTimeStamper::time_stamp();

			accum += (now - stamp).abs();
			assert!((now - stamp).abs() < single_tolerance);
		}

		let average = accum / 100;
		assert!((accum / 100) < tolerance);
	}
}
