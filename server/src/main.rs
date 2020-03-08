use rustarena_lib::packet::SCPacket;
use rustarena_lib::net::Listener;

fn main() {
	let mut listener = Listener::bind("0.0.0.0:4243");
	let mut v = [
		listener.accept_blocking(),
		listener.accept_blocking()
	];
	for x in v.iter_mut() {
		x.send(SCPacket::Start);
	}
	loop {
		// TODO main-loop
	}
}
