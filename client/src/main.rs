mod app;
mod cli;

use app::App;

fn main() {
	let ip = cli::get_ip();
	let mut app = App::connect(&ip);
	app.run();
}
