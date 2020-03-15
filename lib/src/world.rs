use crate::geom::{Vec2f, Circle};
use crate::input::{InputState, Key};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
	pub pos: Vec2f,
	pub health: u32,
	pub input_state: InputState,
}

impl Circle for Player {
	fn center(&self) -> Vec2f { self.pos }
	fn radius(&self) -> f32 { 20. }
}

impl Player {
	#[allow(clippy::new_without_default)]
	fn new() -> Player {
		Player {
			pos: Vec2f::default(),
			health: 100,
			input_state: InputState::new(),
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

	pub fn tick(&mut self) {
		for p in &mut self.players[..] {
			p.pos += p.input_state.direction();
			if p.input_state.is_pressed(Key::Q) {
				p.pos = Vec2f::new(200., 200.);
			}
		}
	}

}
