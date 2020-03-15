#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
	pub x: i64,
	pub y: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct World {
	pub players: [Player; 2],
}

impl World {
	#[allow(clippy::new_without_default)]
	pub fn new() -> World {
		World {
			players: [Player { x: 50, y: 20 }, Player { x: 500, y: 20 }]
		}
	}
}
