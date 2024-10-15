pub struct CharContinuousCounter<const C: char> {
	current: usize,
	max: usize,
}

impl<const C: char> CharContinuousCounter<{ C }> {
	pub fn new() -> Self {
		Self { current: 0, max: 0 }
	}

	pub fn input(&mut self, c: char) {
		if C == c {
			self.current += 1;

			if self.current > self.max {
				self.max = self.current
			}
		} else {
			self.current = 0
		}
	}

	pub fn max(&self) -> usize {
		self.max
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl<const C: char> CharContinuousCounter<{ C }> {
		pub fn assert_current(&self, expected: usize) {
			assert_eq!(self.current, expected);
		}

		pub fn assert_max(&self, expected: usize) {
			assert_eq!(self.max, expected);
			assert_eq!(self.max(), expected);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new() {
		let fixture = CharContinuousCounter::<':'>::new();
		fixture.assert_current(0);
		fixture.assert_max(0);
	}

	#[test]
	fn count_test() {
		let mut fixture = CharContinuousCounter::<':'>::new();
		fixture.input(':');
		fixture.assert_current(1);
		fixture.assert_max(1);

		fixture.input(':');
		fixture.assert_current(2);
		fixture.assert_max(2);

		fixture.input('a');
		fixture.assert_current(0);
		fixture.assert_max(2);

		fixture.input(':');
		fixture.assert_current(1);
		fixture.assert_max(2);
	}
}
