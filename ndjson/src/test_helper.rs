mod equivalent_tests;
mod mock_time_stamper;

pub mod test_prelude {
	pub use super::equivalent_tests::*;
	pub use super::mock_time_stamper::{MockTimeStamper, CALL_COUNT, EXPECTED};
}
