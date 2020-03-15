mod player;
pub use player::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct World {
	pub players: [Player; 2],
}

impl World {
	#[allow(clippy::new_without_default)]
	pub fn new() -> World {
		World {
			players: [Player::new(), Player::new()],
		}
	}

	pub fn tick(&mut self) {
		self.tick_player(0);
		self.tick_player(1);
	}

}
