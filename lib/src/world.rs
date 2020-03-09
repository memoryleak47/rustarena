#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub x: i64,
	pub y: i64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
	pub players: [Player; 2],
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player { x: 50, y: 20 }, Player { x: 500, y: 20 }]
		}
	}
}
