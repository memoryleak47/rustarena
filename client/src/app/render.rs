use crate::app::App;

use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, Shape, Color, Transformable};

impl App {
	fn draw_rect(&mut self, pos: (i64, i64), size: (i64, i64), c: Color) {
		let mut s = RectangleShape::new();
		let v = Vector2f::new(pos.0 as f32, pos.1 as f32);
		s.set_position(v);
		s.set_size(Vector2f::new(size.0 as f32, size.1 as f32));
		s.set_fill_color(c);
		self.window.draw(&s);
	}

	pub fn render(&mut self) {
		let pl = self.state.w.players.clone(); // TODO fix clone

		// player bodies
		for p in pl.iter() {
			self.draw_rect((p.x, p.y), (20, 20), Color::rgb(100, 100, 0));
		}

		// health bars
		for p in pl.iter() {
			self.draw_rect((p.x, p.y - 12), ((p.health / 5) as i64, 2), Color::rgb(244, 0, 0));
		}
	}
}
