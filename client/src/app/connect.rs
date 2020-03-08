use sfml::window::{Style, VideoMode};
use sfml::graphics::{RenderWindow};

use rustarena_lib::{World, Player};
use rustarena_lib::packet::SCPacket;
use rustarena_lib::net::Stream;
use crate::app::App;

impl App {
	pub fn connect(ip: &str) -> App {
		let mut stream = Stream::connect(&*ip);

		match stream.receive_blocking() {
			SCPacket::Start => {},
			_ => panic!("got wrong packet in lobby!"),
		}
		println!("game starts!");

		let world = World {
			players: [Player { x: 50, y: 20 }, Player { x: 500, y: 20 }]
		};

		App {
			world,
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Rustarena client", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			stream,
		}
	}
}
