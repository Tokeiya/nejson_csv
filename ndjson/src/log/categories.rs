#[derive(Clone, Copy)]
pub enum Categories {
	Error,
	Warning,
	Notify,
	Info,
	Verbose,
}

#[cfg(test)]
pub mod test_helper {
	use super::*;
	pub static CATEGORIES: [Categories; 5] = [
		Categories::Error,
		Categories::Warning,
		Categories::Notify,
		Categories::Info,
		Categories::Verbose,
	];
}
