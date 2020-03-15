use sfml::system::{Vector2f, Vector2u};
use sfml::graphics::{RenderTarget, RenderWindow, CircleShape, RectangleShape, Shape, Color, Transformable};

// (0, 0) is left-top

use rustarena_lib::geom::{Vec2f, Circle};

pub trait Window {
	fn draw_circle<T: Circle>(&mut self, circle: &T, c: Color);
	fn draw_rect<V1: Into<Vec2f>, V2: Into<Vec2f>>(&mut self, pos: V1, size: V2, c: Color);
}

fn ratio(w: &RenderWindow) -> f32 { w.size().y as f32 }

fn to_window_pos(w: &RenderWindow, p: Vec2f) -> Vec2f {
	assert_eq!(w.size(), Vector2u::new(1920, 1080)); // TODO build for all screens!
	Vec2f::new(p.x * ratio(w), p.y * ratio(w))
}

fn to_window_size(w: &RenderWindow, s: Vec2f) -> Vec2f {
	assert!(s.x >= 0.);
	assert!(s.y >= 0.);
	s * ratio(w)
}

fn to_window_radius(w: &RenderWindow, r: f32) -> f32 {
	assert!(r >= 0.);
	r * ratio(w)
}

impl Window for RenderWindow {
	fn draw_circle<T: Circle>(&mut self, circle: &T, c: Color) {
		let pos = circle.center();
		let radius = circle.radius();

		let window_pos = to_window_pos(self, pos - radius);
		let window_radius = to_window_radius(self, circle.radius());

		let detail = 20;
		let mut s = CircleShape::new(window_radius, detail);
		s.set_position(Vector2f::new(window_pos.x, window_pos.y));
		s.set_fill_color(c);
		self.draw(&s);
	}

	fn draw_rect<V1: Into<Vec2f>, V2: Into<Vec2f>>(&mut self, pos: V1, size: V2, c: Color) {
		let pos = pos.into();
		let size = size.into();

		let window_pos = to_window_pos(self, pos - size/2.);
		let window_size = to_window_size(self, size);

		let mut s = RectangleShape::new();
		s.set_position(Vector2f::new(window_pos.x, window_pos.y));
		s.set_size(Vector2f::new(window_size.x, window_size.y));
		s.set_fill_color(c);
		self.draw(&s);
	}
}
