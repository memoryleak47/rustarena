mod connect;
mod render;

use sfml::window::{Event, Key};
use sfml::graphics::{RenderWindow, RenderTarget, Color};

use rustarena_lib::input::InputState;
use rustarena_lib::state::State;
use rustarena_lib::net::Stream;
use rustarena_lib::packet::{CSPacket, SCPacket};

pub struct App {
	window: RenderWindow,
	state: State,
	player_id: usize,
	stream: Stream,
}

impl App {
	fn tick(&mut self) {
		let new_is = InputState::build_from_keyboard();
		if new_is != self.state.input_states[self.player_id] {
			self.stream.send(CSPacket::InputStateUpdate(new_is.clone()));
			self.state.input_states[self.player_id] = new_is;
		}
		match self.stream.receive_nonblocking::<SCPacket>() {
			Some(SCPacket::Start { .. }) => { panic!("got start packet while in game!"); }
			Some(SCPacket::StateUpdate(x)) => { self.state = x; }
			None => {},
		}
		self.state.tick();
	}

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

