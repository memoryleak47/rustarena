#![deny(bare_trait_objects)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod packet;
pub mod net;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub x: u64,
	pub y: u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
	pub players: [Player; 2],
}
