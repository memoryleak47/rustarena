mod connect;
mod tick;
mod window;
mod render;

use sfml::window::{Event, Key};
use sfml::graphics::{RenderWindow, RenderTarget, Color};

use rustarena_lib::world::World;
use rustarena_lib::net::Stream;

pub struct App {
	window: RenderWindow,
	world: World,
	player_id: usize,
	stream: Stream,
}

impl App {
	pub fn run(&mut self) {
		while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
				if let Event::KeyPressed { code: Key::Escape, .. } | Event::Closed = event {
					self.window.close();
				}
			}

			self.tick();
			self.render();

			self.window.display();
			self.window.clear(Color::rgb(0, 0, 0));

			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}
}

