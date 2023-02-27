use crate::net::UdpSocket;
use raklib_std::packet::PacketEncode;
use std::net::SocketAddr;

mod error;
mod ping;

pub use error::Error;

pub use ping::ping;

pub struct Client {
    pub socket: UdpSocket,
    addr: SocketAddr,
}

impl Client {
    pub async fn connect<A: tokio::net::ToSocketAddrs>(address: A) -> Result<Self, Error> {
        let addr = crate::net::lookup_host(address).await?;

        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        Ok(Self { socket, addr })
    }

    pub async fn send<T: PacketEncode>(&mut self, packet: &T) -> std::io::Result<usize> {
        self.socket.send(packet, self.addr).await
    }
}
