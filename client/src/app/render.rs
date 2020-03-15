use crate::app::App;
use rustarena_lib::vec::Vec2f;

use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, Shape, Color, Transformable};

impl App {
	fn draw_rect<V1: Into<Vec2f>, V2: Into<Vec2f>>(&mut self, pos: V1, size: V2, c: Color) {
		let pos = pos.into();
		let size = size.into();

		let mut s = RectangleShape::new();
		s.set_position(Vector2f::new(pos.x, pos.y));
		s.set_size(Vector2f::new(size.x, size.y));
		s.set_fill_color(c);
		self.window.draw(&s);
	}

	pub fn render(&mut self) {
		let pl = self.world.players.clone(); // TODO fix clone

		// player bodies
		for p in pl.iter() {
			self.draw_rect(p.pos, (20., 20.), Color::rgb(100, 100, 0));
		}

		// health bars
		for p in pl.iter() {
			self.draw_rect(p.pos - Vec2f::new(0., 12.), (p.health as f32 / 5., 2.), Color::rgb(244, 0, 0));
		}
	}
}
