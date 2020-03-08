mod render;

use sfml::window::{Style, VideoMode, Event, Key};
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use rustarena_lib::{World, Player};

pub struct App {
	window: RenderWindow,
	world: World,
}

impl App {
	pub fn new() -> App {
		let world = World {
			players: [Player { x: 50, y: 20 }, Player { x: 500, y: 20 }]
		};

		App {
			world,
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Rustarena client", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
		}
	}

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

