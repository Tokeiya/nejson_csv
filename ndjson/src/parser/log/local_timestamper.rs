use super::time_stamper::TimeStamper;
use chrono::{DateTime, Local};

pub struct LocalTimeStamper;

impl TimeStamper<Local> for LocalTimeStamper {
	fn time_stamp() -> DateTime<Local> {
		Local::now()
	}
}

#[cfg(test)]
mod tests {
	use super::LocalTimeStamper;
	use crate::parser::log::time_stamper::TimeStamper;
	use chrono::Local;

	#[test]
	fn test_local_time_stamper() {
		let epsilon = chrono::Duration::milliseconds(100);
		let mut cnt = 0usize;

		for _ in 0..100 {
			let expected = Local::now();
			let actual = LocalTimeStamper::time_stamp();
			if (expected - actual).abs() > epsilon {
				cnt += 1;
			}
		}

		assert!(cnt <= 10)
	}
}
