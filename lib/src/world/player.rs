use crate::geom::{Vec2f, Circle};
use crate::input::{InputState, Key};
use crate::world::World;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
	pub pos: Vec2f,
	pub health: u32,
	pub input_state: InputState,
	pub q_skill: Option<Bullet>,
}

impl Circle for Player {
	fn center(&self) -> Vec2f { self.pos }
	fn radius(&self) -> f32 { 20. }
}

impl Player {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Player {
		Player {
			pos: Vec2f::default(),
			health: 100,
			input_state: InputState::new(),
			q_skill: None,
		}
	}
}

impl World {
	pub fn tick_player(&mut self, id: usize) {
		let p = &mut self.players[id];

		p.pos += p.input_state.direction();
		if p.input_state.is_pressed(Key::Q) && p.q_skill.is_none() {
			p.q_skill = Some(Bullet::spawn(p.pos, p.input_state.direction()));
		}

		if let Some(ref mut b) = p.q_skill {
			b.pos += b.direction;
			if b.pos.magnitude() > 100. { p.q_skill = None; }
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bullet {
	pos: Vec2f,
	direction: Vec2f,
}

impl Circle for Bullet {
	fn center(&self) -> Vec2f { self.pos }
	fn radius(&self) -> f32 { 15. }
}

impl Bullet {
	fn spawn(pos: Vec2f, direction: Vec2f) -> Bullet {
		Bullet { pos, direction }
	}
}
