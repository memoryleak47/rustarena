use rustarena_lib::input::InputState;
use rustarena_lib::packet::{CSPacket, SCPacket};

use crate::app::App;

impl App {
	pub fn tick(&mut self) {
		let new_is = InputState::build_from_keyboard();
		let app_is = &mut self.world.players[self.player_id].input_state;
		if new_is != *app_is {
			*app_is = new_is;
			self.stream.send(CSPacket::InputStateUpdate(app_is.clone()));
		}
		match self.stream.receive_nonblocking::<SCPacket>() {
			Some(SCPacket::Start { .. }) => { panic!("got start packet while in game!"); }
			Some(SCPacket::WorldUpdate(w)) => { self.world = w; }
			None => {},
		}
		self.world.tick();
	}
}
