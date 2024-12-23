use std::net::SocketAddr;

use raklib_std::{packet::Packet, stream::BinaryStream};

use crate::{
    protocol::{consts, packets::offline::*},
    server::{session::Session, Result},
};

impl super::Listener {
    pub(super) async fn handle_unconnected(
        &mut self,
        packet_id: u8,
        bstream: &mut BinaryStream,
        addr: SocketAddr,
    ) -> Result<()> {
        let socket = &self.socket;
        match packet_id {
            OfflinePingPacket::ID => {
                let offline_packet = bstream.decode::<OfflinePingPacket>()?;

                let reply = OfflinePongPacket::new(offline_packet.time, consts::SERVER_TITLE);

                socket.send(&reply, addr).await?;
            }
            FirstOpenConnectionRequest::ID => {
                let request = bstream.decode::<FirstOpenConnectionRequest>().unwrap();
                tracing::debug!(mtu.length = request.mtu_length);

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
                let request2 = bstream.decode::<SecondOpenConnectionRequest>().unwrap();
                let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                socket.send(&reply2, addr).await?;
                tracing::info!("new session");

                let (connected_tx, connected_rx) = tokio::sync::mpsc::channel(2048);
                let session = Session::new(addr, connected_tx, socket.clone());
                self.sessions.insert(addr, session);

                // notify about new connection
                self.sender.send((connected_rx, addr)).await.unwrap();
            }
            _ => {
                tracing::error!(packet.id = format!("0x{:02X}", packet_id), "Unknown packet");
            }
        }

        Ok(())
    }
}
