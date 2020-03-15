use std::env;

pub fn get_ip() -> String {
	let args: Vec<String> = env::args().collect();

	match &args[..] {
		[_, x] => (*x).to_string(),
		_ => panic!("invalid number of CLI parameters"),
	}
}
