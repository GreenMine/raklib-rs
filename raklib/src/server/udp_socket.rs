use crate::*;
use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket as RawUdpSocket},
    ops::Deref,
};

use raklib_std::packet::PacketEncode;

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
        packet: &T,
        addr: A,
    ) -> std::io::Result<usize> {
        {
            //FIXME: do it only in debug mode
            let full_packet_name = std::any::type_name_of_val(packet);
            let packet_name = full_packet_name
                .split("::")
                .last()
                .unwrap_or(full_packet_name);

            debug!("Send {} packet!", packet_name);
        }

        self.socket.send_to(packet.encode().get_raw(), addr)
    }
}

impl Deref for UdpSocket {
    type Target = RawUdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}
