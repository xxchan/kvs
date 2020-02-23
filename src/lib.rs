use failure::Error;
use std::result;
#[macro_use]
extern crate log;

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use server::KvsServer;

mod client;
mod engines;
mod server;
mod protocol;

pub type Result<T> = result::Result<T, Error>;
