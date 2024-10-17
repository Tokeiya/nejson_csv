use crate::parser::log::time_stamper::TimeStamper;
use chrono::prelude::*;
use std::marker::PhantomData;

pub enum LogCategories {
	Error,
	Warning,
	Info,
	Verbose,
}

pub struct LogDatum<Tz: TimeZone, Ts: TimeStamper<Tz>> {
	time_stamp: DateTime<Tz>,
	category: LogCategories,
	message: String,
	phantom: PhantomData<Ts>,
}

impl<Tz: TimeZone, Ts: TimeStamper<Tz>> LogDatum<Tz, Ts> {
	pub fn new(category: LogCategories, message: String) -> Self {
		todo!()
	}

	pub fn category(&self) -> &LogCategories {
		todo!()
	}

	pub fn time_stamp(&self) -> &DateTime<Tz> {
		todo!()
	}

	pub fn message(&self) -> &str {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::LogDatum;
	use chrono::Local;
	use chrono::TimeZone;
	use std::marker::PhantomData;
}
