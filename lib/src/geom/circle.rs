use crate::geom::Vec2f;

pub trait Circle {
	fn center(&self) -> Vec2f;
	fn radius(&self) -> f32;
	fn collide<T: Circle>(&self, other: &T) -> bool {
		let r = self.radius() + other.radius();
		self.center().distance_sqr(other.center()) <= r * r
	}
}
