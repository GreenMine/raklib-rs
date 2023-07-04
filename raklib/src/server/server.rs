use std::net::SocketAddr;

use crate::{
    dialogue::{Dialogue, DialogueHandler},
    net::{Error as NetError, UdpSocket},
    session::Session,
    protocol::{consts, packets::offline::*}
};
use super::Sessions;
use raklib_std::packet::Packet;

use tokio::sync::mpsc;
use dashmap::mapref::one::RefMut;


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
        let handler = ServerHandler {
            sessions: dashmap::DashMap::new(),
        };

        tokio::spawn(Dialogue::new(handler, self.socket).run());

        Ok(())
    }
}

struct ServerHandler {
    sessions: Sessions,
}

impl DialogueHandler for ServerHandler {
    type SessionRef<'a> = RefMut<'a, SocketAddr, Session>;

    fn get_session(&self, addr: SocketAddr) -> Option<Self::SessionRef<'_>> {
        self.sessions.get_mut(&addr)
    }

    async fn tick(&self) {
        // Update session:
        //  Sessions::retain is no useful, first of all because we awaiting session update,
        //  and because of amount of disconnected session will not be a lot.
        for mut session in self.sessions.iter_mut() {
            session.update().await;

            if !session.status.is_connected() {
                let session_addr = session.get_addr();
                drop(session);

                self.sessions.remove(&session_addr);
            }
        }
    }

    async fn unconnected(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet_id: u8,
        bstream: &mut raklib_std::stream::BinaryStream,
    ) -> crate::dialogue::Result<()> {
        match packet_id {
            OfflinePingPacket::ID => {
                let offline_packet = bstream.decode::<OfflinePingPacket>()?;

                let reply = OfflinePongPacket::new(offline_packet.time, consts::SERVER_TITLE);

                socket.send(&reply, addr).await?;
            }
            FirstOpenConnectionRequest::ID => {
                let request = bstream.decode::<FirstOpenConnectionRequest>().unwrap();
                //TODO: protocol acceptor
                if request.protocol_version != consts::PROTOCOL_VERSION {
                    socket
                        .send(&IncompatibleProtocolVersion::new(), addr)
                        .await?;
                } else {
                    socket
                        .send(
                            &FirstOpenConnectionReply::new(false, request.mtu_length),
                            addr,
                        )
                        .await?;
                }
            }
            SecondOpenConnectionRequest::ID => {
                // TODO: save mtu_length
                let request2 = bstream.decode::<SecondOpenConnectionRequest>().unwrap();
                let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                socket.send(&reply2, addr).await?;
                log::info!("Create new session for {}!", addr);

                let (connected_tx, connected_rx) = mpsc::channel(64);
                let session = Session::new(addr, connected_tx, socket.clone());
                self.sessions.insert(addr, session);

                // notify about new connection
                // TODO: sender.send((addr, connected_rx)).await.unwrap();
            }
            _ => {
                log::error!("Unimplemented packet: 0x{:02X}", packet_id,);
            }
        }

        Ok(())
    }
}
