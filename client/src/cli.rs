use std::env;

#[derive(Default)]
pub struct Config {
	pub ip: String,
	pub bot: bool,
}

impl Config {
	pub fn load() -> Config {
		let mut c = Config::default();

		for x in env::args().skip(1) {
			if &x == "--bot" {
				c.bot = true;
			} else if c.ip.is_empty() {
				c.ip = x;
			} else {
				panic!("too many command line arguments");
			}
		}

		c
	}
}
