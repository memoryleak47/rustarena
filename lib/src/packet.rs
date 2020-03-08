use serde::{Serialize, de::DeserializeOwned};

// Server -> Client Packet
#[derive(Serialize, Deserialize, Clone)]
pub enum SCPacket {
	Start
}

// Client -> Server Packet
#[derive(Serialize, Deserialize, Clone)]
pub enum CSPacket { }

pub trait Packet: Serialize + DeserializeOwned {}

impl Packet for SCPacket {}
impl Packet for CSPacket {}
