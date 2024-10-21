use crate::log::prelude::TimeStamper;
use chrono::{DateTime, Local};
use std::sync::atomic::AtomicUsize;
use std::sync::LazyLock;

#[cfg(test)]
pub static EXPECTED: LazyLock<DateTime<Local>> = LazyLock::new(|| {
	std::thread::sleep(std::time::Duration::from_micros(100));
	Local::now()
});

#[cfg(test)]
pub static CALL_COUNT: AtomicUsize = AtomicUsize::new(0usize);

#[cfg(test)]
pub struct MockTimeStamper;

#[cfg(test)]
impl TimeStamper<Local> for MockTimeStamper {
	fn time_stamp() -> DateTime<Local> {
		CALL_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		*EXPECTED
	}
}
