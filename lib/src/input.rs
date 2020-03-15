use std::collections::HashSet;

use crate::vec::Vec2f;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
	W, A, S, D,
	Q,
}

const ALL_KEYS: [Key; 5] = [Key::W, Key::A, Key::S, Key::D, Key::Q];

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct InputState {
	set: HashSet<Key>,
}

impl InputState {
	#[allow(clippy::new_without_default)]
	pub fn new() -> InputState {
		InputState { set: HashSet::new() }
	}

	pub fn build_from_keyboard() -> InputState {
		let mut set: HashSet<Key> = HashSet::new();
		for &x in &ALL_KEYS[..] {
			if x.to_sfml_key().is_pressed() {
				set.insert(x);
			}
		}
		InputState { set }
	}

	pub fn is_pressed(&self, k: Key) -> bool {
		self.set.contains(&k)
	}

	pub fn direction(&self) -> Vec2f {
		let x = self.is_pressed(Key::D) as i32 - self.is_pressed(Key::A) as i32;
		let y = self.is_pressed(Key::S) as i32 - self.is_pressed(Key::W) as i32;
		Vec2f::new(x as f32, y as f32)
	}
}

impl Key {
	fn to_sfml_key(self) -> sfml::window::Key {
		match self {
			Key::W => sfml::window::Key::W,
			Key::A => sfml::window::Key::A,
			Key::S => sfml::window::Key::S,
			Key::D => sfml::window::Key::D,
			Key::Q => sfml::window::Key::Q,
		}
	}
}
