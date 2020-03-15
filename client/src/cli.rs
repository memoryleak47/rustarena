use std::env;

#[derive(Default)]
pub struct Config {
	pub ip: String,
	pub bot: bool,
}

impl Config {
	pub fn load() -> Config {
		let mut args = env::args().skip(1);
		let mut c = Config::default();

		while let Some(x) = args.next() {
			if &x == "--bot" {
				c.bot = true;
			} else {
				if c.ip.is_empty() {
					c.ip = x;
				} else {
					panic!("too many command line arguments");
				}
			}
		}

		c
	}
}
