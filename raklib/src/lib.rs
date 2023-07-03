#![feature(type_name_of_val, async_fn_in_trait)]

pub mod net;
//FIXME: pub(crate)
// pub use udp_socket::UdpSocket;

pub mod client;
pub mod protocol;
pub mod server;

pub mod dialogue;
