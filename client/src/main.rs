mod cli;
mod app;
mod bot;

use cli::Config;
use app::App;

fn main() {
	let config = Config::load();
	if config.bot {
		bot::run(&config.ip);
	} else {
		let mut app = App::connect(&config.ip);
		app.run();
	}
}
