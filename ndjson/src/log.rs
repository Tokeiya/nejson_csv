mod categories;
mod console_logger;
mod local_time_stamper;
mod log_datum;
mod logger;
mod text_logger;
mod time_stamper;

pub mod prelude {
	pub use super::categories::Categories;
	pub use super::console_logger::ConsoleLogger;
	pub use super::local_time_stamper::LocalTimeStamper;
	pub use super::log_datum::LogDatum;
	pub use super::logger::Logger;
	pub use super::time_stamper::TimeStamper;
}

#[cfg(test)]
pub mod test_prelude {
	pub use super::categories::test_helper as test_categories;
	pub use super::logger::test_helper as test_logger;
}
