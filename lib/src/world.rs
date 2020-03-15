#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
	pub x: i64,
	pub y: i64,
	pub health: u32,
}

impl Player {
	#[allow(clippy::new_without_default)]
	fn new() -> Player {
		Player {
			x: 0,
			y: 0,
			health: 100,
		}
	}
}

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
}
