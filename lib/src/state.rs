use crate::input::InputState;
use crate::world::World;

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
	pub w: World,
	pub input_states: [InputState; 2]
}

impl State {
	#[allow(clippy::new_without_default)]
	pub fn new() -> State {
		State { w: World::new(), input_states: [InputState::new(), InputState::new()] }
	}

	pub fn tick(&mut self) {
		for i in 0..2 {
			self.w.players[i].x += self.input_states[i].dirx();
			self.w.players[i].y += self.input_states[i].diry();
		}
	}
}
