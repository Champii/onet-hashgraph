#![feature(duration_as_u128)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rsrpc;
#[macro_use]
pub extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use(defer)]
extern crate scopeguard;

extern crate bincode;
// extern crate env_logger;
extern crate rand;
extern crate ring;
extern crate serde;
extern crate serde_bytes;
extern crate untrusted;

mod event;
mod events;
mod hashgraph;
mod hashgraph_tests;
mod internal_txs;
mod key;
// pub mod logger;
mod node;
mod peer;
mod peers;
mod round;
mod rpc;

pub use key::Key;
pub use node::{Node, NodeConfig};
pub use peer::Peer;

#[macro_export]
macro_rules! trace_time {
    ($expr:expr) => {
        let now = SystemTime::now();

        defer!(trace!("Time: {}: {:?}", $expr, now.elapsed().unwrap()));
    };
}
