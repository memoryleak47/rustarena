use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt::{Display, Debug, Error, Formatter};

use crate::WORLD_SIZE;

#[derive(Serialize, Deserialize)]
#[derive(Default, PartialEq, Clone, Copy)]
pub struct Vec2f {
	pub x: f32,
	pub y: f32,
}

impl Vec2f {
	pub const fn new(x: f32, y: f32) -> Vec2f {
		Vec2f { x, y }
	}

	pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Vec2f {
		Vec2f::new(
			f(self.x),
			f(self.y),
		)
	}

	pub fn distance(self, other: Vec2f) -> f32 {
		(self - other).magnitude()
	}

	pub fn distance_sqr(self, other: Vec2f) -> f32 {
		(self - other).magnitude_sqr()
	}

	pub fn magnitude(self) -> f32 {
		(self.magnitude_sqr()).sqrt()
	}

	pub fn magnitude_sqr(self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	pub fn in_world(self) -> bool {
		(0. <= self.x) && (self.x <= WORLD_SIZE.x) &&
		(0. <= self.y) && (self.y <= WORLD_SIZE.y)
	}

	pub fn crop_world(self) -> Vec2f {
		let mut x = self.x;
		if x < 0. { x = 0.; }
		else if x > WORLD_SIZE.x { x = WORLD_SIZE.x; }

		let mut y = self.y;
		if y < 0. { y = 0.; }
		else if y > WORLD_SIZE.y { y = WORLD_SIZE.y; }

		Vec2f::new(x, y)
	}
}

impl From<(f32, f32)> for Vec2f {
	fn from(item: (f32, f32)) -> Vec2f {
		Vec2f::new(item.0, item.1)
	}
}

impl From<f32> for Vec2f {
	fn from(item: f32) -> Vec2f {
		Vec2f::new(item, item)
	}
}

impl Display for Vec2f {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2f({}, {})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

impl Debug for Vec2f {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let s = format!("Vec2f({:?}, {:?})", self.x, self.y);
		fmt.write_str(&*s)
	}
}

// operator implementations:

// add
impl<T: Into<Vec2f>> Add<T> for Vec2f {
	type Output = Vec2f;

	fn add(self, other: T) -> Vec2f {
		let other = other.into();
		Vec2f::new (
			self.x + other.x,
			self.y + other.y,
		)
	}
}

impl<T: Into<Vec2f>> AddAssign<T> for Vec2f {
	fn add_assign(&mut self, other: T) {
		*self = *self + other.into();
	}
}

// sub
impl<T: Into<Vec2f>> Sub<T> for Vec2f {
	type Output = Vec2f;

	fn sub(self, other: T) -> Vec2f {
		let other = other.into();
		Vec2f::new (
			self.x - other.x,
			self.y - other.y,
		)
	}
}

impl<T: Into<Vec2f>> SubAssign<T> for Vec2f {
	fn sub_assign(&mut self, other: T) {
		*self = *self - other.into();
	}
}

// mul
impl<T: Into<Vec2f>> Mul<T> for Vec2f {
	type Output = Vec2f;

	fn mul(self, other: T) -> Vec2f {
		let other = other.into();
		Vec2f::new (
			self.x * other.x,
			self.y * other.y,
		)
	}
}

impl<T: Into<Vec2f>> MulAssign<T> for Vec2f {
	fn mul_assign(&mut self, other: T) {
		*self = *self * other.into();
	}
}

// div
impl<T: Into<Vec2f>> Div<T> for Vec2f {
	type Output = Vec2f;

	fn div(self, other: T) -> Vec2f {
		let other = other.into();
		Vec2f::new (
			self.x / other.x,
			self.y / other.y,
		)
	}
}

impl<T: Into<Vec2f>> DivAssign<T> for Vec2f {
	fn div_assign(&mut self, other: T) {
		*self = *self / other.into();
	}
}
