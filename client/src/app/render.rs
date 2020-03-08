use crate::app::App;

use sfml::system::Vector2f;
use sfml::graphics::{RenderTarget, RectangleShape, Shape, Color, Transformable};

impl App {
	pub fn render(&mut self) {
		for p in self.world.players.iter() {
			let mut s = RectangleShape::new();
			let v = Vector2f::new(p.x as f32, p.y as f32);
			s.set_position(v);
			s.set_size(Vector2f::new(20., 20.));
			s.set_fill_color(Color::rgb(100, 100, 0));
			self.window.draw(&s);
		}
	}
}
