use serde::{Serialize, de::DeserializeOwned};
use bincode::{serialize, deserialize};

pub fn ser<P: Serialize>(p: P) -> Vec<u8> {
	serialize(&p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
