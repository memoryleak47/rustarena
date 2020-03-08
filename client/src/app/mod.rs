mod connect;
mod render;

use sfml::window::{Event, Key};
use sfml::graphics::{RenderWindow, RenderTarget, Color};

use rustarena_lib::World;
use rustarena_lib::net::Stream;

pub struct App {
	window: RenderWindow,
	world: World,
	stream: Stream,
}

impl App {
	fn tick(&mut self) {} // TODO

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

