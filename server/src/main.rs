use rustarena_lib::packet::{CSPacket, SCPacket};
use rustarena_lib::net::Listener;
use rustarena_lib::state::State;

fn main() {
	let mut listener = Listener::bind("0.0.0.0:4243");
	let mut v = [
		listener.accept_blocking(),
		listener.accept_blocking()
	];
	let mut state = State::new();
	for (i, x) in v.iter_mut().enumerate() {
		x.send(SCPacket::Start { state: state.clone(), player_id: i as usize });
	}
	loop {
		let mut changed = false;
		for (i, l) in v.iter_mut().enumerate() {
			#[allow(clippy::single_match)]
			match l.receive_nonblocking() {
				Some(CSPacket::InputStateUpdate(is)) => {
					state.input_states[i] = is;
					changed = true;
				}
				None => {},
			}
		}
		if changed {
			for l in v.iter_mut() {
				l.send(SCPacket::StateUpdate(state.clone()));
			}
		}
		state.tick();

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
