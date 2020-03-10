use rustarena_lib::input::InputState;
use rustarena_lib::packet::{CSPacket, SCPacket};

use crate::app::App;

impl App {
	pub fn tick(&mut self) {
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
}
