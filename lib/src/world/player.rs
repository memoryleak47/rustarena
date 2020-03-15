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
	fn radius(&self) -> f32 { 0.05 }
}

impl Player {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Player {
		Player {
			pos: Vec2f::new(0.3, 0.3),
			health: 100,
			input_state: InputState::new(),
			q_skill: None,
		}
	}
}

impl World {
	pub fn tick_player(&mut self, id: usize) {
		let [p0, p1] = &mut self.players;

		let (me, enemy) = if id == 0 {
			(p0, p1)
		} else {
			(p1, p0)
		};

		me.pos += me.input_state.direction() * 0.01;

		if me.input_state.is_pressed(Key::Q) && me.q_skill.is_none() {
			me.q_skill = Some(Bullet::spawn(me.pos, me.input_state.direction()));
		}

		if let Some(ref mut b) = me.q_skill {
			b.pos += b.direction * 0.02;
			if !b.pos.in_world() { me.q_skill = None; }
			else if b.collide(enemy) {
				me.q_skill = None;
				enemy.health -= 10;
			}
		}

		me.pos = me.pos.crop_world();
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bullet {
	pos: Vec2f,
	direction: Vec2f,
}

impl Circle for Bullet {
	fn center(&self) -> Vec2f { self.pos }
	fn radius(&self) -> f32 { 0.02 }
}

impl Bullet {
	fn spawn(pos: Vec2f, direction: Vec2f) -> Bullet {
		Bullet { pos, direction }
	}
}
