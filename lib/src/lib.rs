#![deny(bare_trait_objects)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod input;
pub mod world;
pub mod packet;
pub mod net;
