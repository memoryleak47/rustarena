use sfml::graphics::Color;

use rustarena_lib::geom::Circle;

use crate::app::App;
use crate::app::window::Window;

impl App {
	pub fn render(&mut self) {
		// player bodies
		for p in self.world.players.iter() {
			self.window.draw_circle(p, Color::rgb(100, 100, 0));
		}

		// q_skill rendering
		for p in self.world.players.iter() {
			if let Some(ref b) = p.q_skill {
				self.window.draw_circle(b, Color::rgb(0, 0, 80));
			}
		}

		// health bars
		for p in self.world.players.iter() {
			self.window.draw_rect(p.center() - (0., p.radius() * 1.2), (p.health as f32 / 100. * 2. * p.radius(), 0.004), Color::rgb(244, 0, 0));
		}
	}
}
