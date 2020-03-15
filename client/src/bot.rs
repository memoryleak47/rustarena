use rustarena_lib::packet::SCPacket;
use rustarena_lib::net::Stream;

pub fn run(ip: &str) {
	let mut stream = Stream::connect(&*ip);

	loop {
		println!("{:?}", stream.receive_blocking::<SCPacket>());
	};
}
