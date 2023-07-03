use std::net::SocketAddr;
use dashmap::mapref::one::RefMut;
use crate::{net::{Error as NetError, UdpSocket}, dialogue::{DialogueHandler, Dialogue}, session::Session};

use super::Sessions;

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
        let handler = ServerHandler { sessions: dashmap::DashMap::new() };

        tokio::spawn(Dialogue::new(handler, self.socket).run());

        Ok(())
    }

}

struct ServerHandler {
    sessions: Sessions
}

impl DialogueHandler for ServerHandler {
    type SessionRef<'a> = RefMut<'a, SocketAddr, Session>;

    fn get_session(&self, addr: SocketAddr) -> Option<Self::SessionRef<'_>> {
        self.sessions.get_mut(&addr)
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
