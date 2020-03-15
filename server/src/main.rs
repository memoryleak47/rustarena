use rustarena_lib::packet::{CSPacket, SCPacket};
use rustarena_lib::net::Listener;
use rustarena_lib::world::World;

fn main() {
	let mut listener = Listener::bind("0.0.0.0:4243");
	let mut v = [
		listener.accept_blocking(),
		listener.accept_blocking()
	];
	let mut world = World::new();
	for (i, x) in v.iter_mut().enumerate() {
		x.send(SCPacket::Start { world: world.clone(), player_id: i as usize });
	}
	loop {
		let mut changed = false;
		for (i, l) in v.iter_mut().enumerate() {
			#[allow(clippy::single_match)]
			match l.receive_nonblocking() {
				Some(CSPacket::InputStateUpdate(is)) => {
					world.players[i].input_state = is;
					changed = true;
				}
				None => {},
			}
		}
		if changed {
			for l in v.iter_mut() {
				l.send(SCPacket::WorldUpdate(world.clone()));
			}
		}
		world.tick();

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
