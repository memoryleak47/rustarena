use serde::{Serialize, de::DeserializeOwned};

use crate::input::InputState;
use crate::world::World;

// Server -> Client Packet
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SCPacket {
	Start { world: World, player_id: usize },
	WorldUpdate(World),
}

// Client -> Server Packet
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CSPacket {
	InputStateUpdate(InputState)
}

pub trait Packet: Serialize + DeserializeOwned {}

impl Packet for SCPacket {}
impl Packet for CSPacket {}
