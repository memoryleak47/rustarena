use crate::geom::Vec2f;

pub trait Circle {
	fn center(&self) -> Vec2f;
	fn radius(&self) -> f32;
}

pub fn circle_collide<T1: Circle, T2: Circle>(a: T1, b: T2) -> bool {
	let r = a.radius() + b.radius();
	a.center().distance_sqr(b.center()) <= r * r
}
