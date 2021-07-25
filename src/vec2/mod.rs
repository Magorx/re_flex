use std::ops;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {x, y}
	}
}

impl<T: ops::Add<Output = T>> ops::Add for Vec2<T> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {x: self.x + other.x, y: self.y + other.y}
	}
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec2<T> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {x: self.x - other.x, y: self.y - other.y}
	}
}

impl<T: ops::Mul<T, Output = T>> ops::Mul<Vec2<T>> for Vec2<T> {
	type Output = Self;

	fn mul(self, other: Vec2<T>) -> Self {
		Self {x: self.x * other.x, y: self.y * other.y}
	}
}

impl<T: ops::Mul<f32, Output = T>> ops::Mul<f32> for Vec2<T> {
	type Output = Self;

	fn mul(self, other: f32) -> Self {
		Self {x: self.x * other, y: self.y * other}
	}
}

impl<T: ops::Mul<f32, Output = T>> ops::Mul<Vec2<T>> for f32 {
	type Output = Vec2<T>;

	fn mul(self, other: Vec2<T>) -> Vec2<T> {
		Vec2 {x: other.x * self, y: other.y * self}
	}
}

impl<T: ops::Div<Output = T>> ops::Div for Vec2<T> {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		Self {x: self.x / other.x, y: self.y / other.y}
	}
}

impl<T: ops::Div<f32, Output = T>> ops::Div<f32> for Vec2<T> {
	type Output = Self;

	fn div(self, other: f32) -> Self {
		Self {x: self.x / other, y: self.y / other}
	}
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
