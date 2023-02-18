use std::{net::SocketAddr, ops::Deref};

use tokio::net::{ToSocketAddrs, UdpSocket as RawUdpSocket};

use raklib_std::packet::PacketEncode;

pub(crate) struct UdpSocket {
    address: SocketAddr,
    socket: RawUdpSocket,
}

impl UdpSocket {
    pub(crate) async fn bind(address: SocketAddr) -> std::io::Result<Self> {
        let socket = RawUdpSocket::bind(address).await?;

        Ok(Self { address, socket })
    }

    pub(crate) fn get_bind_address(&self) -> &SocketAddr {
        &self.address
    }

    pub(crate) async fn send<T: PacketEncode, A: ToSocketAddrs>(
        &self,
        packet: &T,
        addr: A,
    ) -> std::io::Result<usize> {
        if log::log_enabled!(log::Level::Debug) {
            let full_packet_name = std::any::type_name_of_val(packet);
            let packet_name = full_packet_name
                .split("::")
                .last()
                .unwrap_or(full_packet_name);

            log::debug!("Send {} packet!", packet_name);
        }

        self.socket.send_to(packet.encode().get_raw(), addr).await
    }
}

impl Deref for UdpSocket {
    type Target = RawUdpSocket;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}
