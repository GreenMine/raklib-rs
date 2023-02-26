use crate::net::UdpSocket;
use raklib_std::packet::PacketEncode;
use std::net::SocketAddr;

mod ping;
pub use ping::ping;

pub struct Client {
    pub socket: UdpSocket,
    addr: SocketAddr,
}

impl Client {
    pub async fn connect(addr: SocketAddr) -> Self {
        let local_addr = "0.0.0.0:0".parse().unwrap();
        let mut socket = UdpSocket::bind(local_addr).await.unwrap();

        Self { socket, addr }
    }

    pub async fn send<T: PacketEncode>(&mut self, packet: &T) -> std::io::Result<usize> {
        self.socket.send(packet, self.addr).await
    }
}
