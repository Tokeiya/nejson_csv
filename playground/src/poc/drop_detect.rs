use std::cell::Cell;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static SEED: AtomicUsize = AtomicUsize::new(0);

pub struct DropDetector(usize);

impl DropDetector {
	pub fn new() -> DropDetector {
		DropDetector(SEED.fetch_add(1, Ordering::SeqCst))
	}

	pub fn identity(&self) -> usize {
		self.0
	}
}

impl Drop for DropDetector {
	fn drop(&mut self) {
		println!("{} is dropping.", self.0);
	}
}
