use sfml::graphics::Color;

use rustarena_lib::geom::{Vec2f, Circle};

use crate::app::App;
use crate::app::window::Window;

impl App {
	pub fn render(&mut self) {
		let pl = self.world.players.clone(); // TODO fix clone

		// player bodies
		for p in pl.iter() {
			self.window.draw_circle(p, Color::rgb(100, 100, 0));
		}

		// q_skill rendering
		for p in pl.iter() {
			if let Some(ref b) = p.q_skill {
				self.window.draw_circle(b, Color::rgb(0, 0, 80));
			}
		}

		// health bars
		for p in pl.iter() {
			self.window.draw_rect(p.center() - Vec2f::new(0., 12.), (p.health as f32 * 2. / 5., 2.), Color::rgb(244, 0, 0));
		}
	}
}
