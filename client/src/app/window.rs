use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RenderWindow, CircleShape, RectangleShape, Shape, Color, Transformable};

// TODO also scale radius, rect size with window

use rustarena_lib::geom::{Vec2f, Circle};

pub trait Window {
	fn draw_circle<T: Circle>(&mut self, circle: &T, c: Color);
	fn draw_rect<V1: Into<Vec2f>, V2: Into<Vec2f>>(&mut self, pos: V1, size: V2, c: Color);
}

fn to_window_coord(w: &RenderWindow, p: Vec2f) -> Vec2f {
	// TODO normalize for all screens!
	Vec2f::new((p.x/100.) * w.size().x as f32, (p.y/100.) * w.size().y as f32)
}

impl Window for RenderWindow {
	fn draw_circle<T: Circle>(&mut self, circle: &T, c: Color) {
		let pos = circle.center();
		let window_pos = to_window_coord(self, pos);

		let mut s = CircleShape::new(circle.radius(), 20);
		s.set_position(Vector2f::new(window_pos.x, window_pos.y));
		s.set_fill_color(c);
		self.draw(&s);
	}

	fn draw_rect<V1: Into<Vec2f>, V2: Into<Vec2f>>(&mut self, pos: V1, size: V2, c: Color) {
		let pos = pos.into();
		let size = size.into();

		let window_pos = to_window_coord(self, pos);

		let mut s = RectangleShape::new();
		s.set_position(Vector2f::new(window_pos.x, window_pos.y));
		s.set_size(Vector2f::new(size.x, size.y));
		s.set_fill_color(c);
		self.draw(&s);
	}
}
