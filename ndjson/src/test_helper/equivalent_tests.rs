use std::cmp::PartialEq;

#[cfg(test)]
pub fn symmetric<T: PartialEq>(x: &T, y: &T) {
	assert!(&x == &y);
	assert!(&y == &x);
}

#[cfg(test)]
pub fn reflexive<T: PartialEq>(x: &T) {
	assert!(&x == &x)
}

#[cfg(test)]
pub fn transitive<T: PartialEq>(x: &T, y: &T, z: &T) {
	assert!(&x == &y);
	assert!(&y == &z);
}

#[cfg(test)]
pub fn not_equal<T: PartialEq>(x: &T, y: &T) {
	assert!(&x != &y);
}

#[cfg(test)]
pub fn equivalent<T: PartialEq>(x: T, y: T, z: T, not_eq: T) {
	symmetric(&x, &y);
	reflexive(&x);
	transitive(&x, &y, &z);
	not_equal(&x, &not_eq);
}
