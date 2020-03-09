use sfml::window::{Style, VideoMode};
use sfml::graphics::{RenderWindow};

use rustarena_lib::packet::SCPacket;
use rustarena_lib::net::Stream;
use crate::app::App;

impl App {
	pub fn connect(ip: &str) -> App {
		let mut stream = Stream::connect(&*ip);

		let (state, player_id) = match stream.receive_blocking() {
			SCPacket::Start { state, player_id } => (state, player_id),
			_ => panic!("got wrong packet in lobby!"),
		};
		println!("game starts!");

		App {
			state,
			player_id,
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Rustarena client", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			stream,
		}
	}
}
