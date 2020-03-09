use serde::{Serialize, de::DeserializeOwned};

use crate::input::InputState;
use crate::state::State;

// Server -> Client Packet
#[derive(Serialize, Deserialize, Clone)]
pub enum SCPacket {
	Start { state: State, player_id: usize },
	StateUpdate(State),
}

// Client -> Server Packet
#[derive(Serialize, Deserialize, Clone)]
pub enum CSPacket {
	InputStateUpdate(InputState)
}

pub trait Packet: Serialize + DeserializeOwned {}

impl Packet for SCPacket {}
impl Packet for CSPacket {}
