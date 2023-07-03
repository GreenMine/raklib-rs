use std::net::SocketAddr;
use dashmap::DashMap;
use crate::{net::{Error as NetError, UdpSocket}, dialogue::{DialogueHandler, Dialogue}};
use super::session::Session;

pub struct Server {
    pub(super) socket: UdpSocket,
}

unsafe impl Send for Server {}

impl Server {
    pub async fn bind(address: SocketAddr) -> Result<Self, NetError> {
        Ok(Self {
            socket: UdpSocket::bind(address).await?,
        })
    }

    pub async fn run(self) -> std::io::Result<()> {
        let handler = ServerHandler { sessions: DashMap::new() };
        Dialogue::new(handler, self.socket).run().await;
        Ok(())
    }

}

struct ServerHandler {
    sessions: DashMap<SocketAddr, Session>
}

impl DialogueHandler for ServerHandler {
    type Session = Session;

    fn get_session(&self, addr: SocketAddr) -> Self::Session {
        todo!()
    }

    fn unconnected(&self, packet: raklib_std::stream::BinaryStream) -> Result<(), ()> {
        todo!()
    }

    fn establish() {
        todo!()
    }

    fn on_packet() {
        todo!()
    }
}
