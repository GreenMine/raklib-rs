use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket as RawUdpSocket},
    ops::Deref,
};

use crate::protocol::packets::PacketEncode;

pub(crate) struct UdpSocket {
    address: SocketAddr,
    socket: RawUdpSocket,
}

impl UdpSocket {
    pub(crate) fn bind(address: SocketAddr) -> std::io::Result<Self> {
        let socket = RawUdpSocket::bind(address)?;
        socket
            .set_nonblocking(true)
            .expect("Error to set UDP socket to non-blocking mode");

        Ok(Self { address, socket })
    }

    pub(crate) fn get_bind_address(&self) -> &SocketAddr {
        &self.address
    }

    pub(crate) fn send<T: PacketEncode, A: ToSocketAddrs>(
        &self,
        packet: T,
        addr: A,
    ) -> std::io::Result<usize> {
        self.socket.send_to(packet.encode().get_raw(), addr)
    }
}

impl Deref for UdpSocket {
    type Target = RawUdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}
