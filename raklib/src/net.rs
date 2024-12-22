use std::{io, net::SocketAddr, ops::Deref};

use tokio::net::{ToSocketAddrs, UdpSocket as RawUdpSocket};

use raklib_std::packet::PacketEncode;

pub struct UdpSocket {
    address: SocketAddr,
    socket: RawUdpSocket,
}

impl UdpSocket {
    pub(crate) async fn bind<A: ToSocketAddrs + Copy>(address: A) -> Result<Self, Error> {
        let socket = RawUdpSocket::bind(address).await?;

        Ok(Self {
            address: lookup_host(address).await?,
            socket,
        })
    }

    pub fn get_bind_address(&self) -> &SocketAddr {
        &self.address
    }

    pub async fn send<T: PacketEncode, A: ToSocketAddrs>(
        &self,
        packet: &T,
        addr: A,
    ) -> std::io::Result<usize> {
        if tracing::enabled!(tracing::Level::TRACE) {
            let full_packet_name = std::any::type_name_of_val(packet);
            let packet_name = full_packet_name
                .split("::")
                .last()
                .unwrap_or(full_packet_name);

            tracing::trace!(packet_name, "send packet");
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("invalid address lookup")]
    Lookup,
}

pub async fn lookup_host<A: ToSocketAddrs>(address: A) -> Result<SocketAddr, Error> {
    tokio::net::lookup_host(address)
        .await?
        .next()
        .ok_or(Error::Lookup)
}
