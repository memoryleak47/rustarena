use std::collections::HashSet;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
	W, A, S, D,
}

const ALL_KEYS: [Key; 4] = [Key::W, Key::A, Key::S, Key::D];

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InputState {
	set: HashSet<Key>,
}

impl InputState {
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

	pub fn dirx(&self) -> i64 {
		let mut r = 0;
		if self.is_pressed(Key::D) { r += 1; }
		if self.is_pressed(Key::A) { r -= 1; }
		r
	}

	pub fn diry(&self) -> i64 {
		let mut r = 0;
		if self.is_pressed(Key::S) { r += 1; }
		if self.is_pressed(Key::W) { r -= 1; }
		r
	}
}

impl Key {
	fn to_sfml_key(&self) -> sfml::window::Key {
		match self {
			Key::W => sfml::window::Key::W,
			Key::A => sfml::window::Key::A,
			Key::S => sfml::window::Key::S,
			Key::D => sfml::window::Key::D,
		}
	}
}
