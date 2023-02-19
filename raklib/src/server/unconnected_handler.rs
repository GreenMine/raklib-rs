use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Sender;

use raklib_std::{packet::Packet, stream::BinaryStream};

use crate::{
    protocol::{consts, packets::offline::*},
    server::session::Session,
};
use crate::server::{ConnectedData, Sessions, UdpSocket};

use super::{Result, Server};

impl Server {
    pub(crate) async fn unconnected_handler(
        socket: &Arc<UdpSocket>,
        sender: &Sender<ConnectedData>,
        sessions: &Arc<Mutex<Sessions>>,
        packet_id: u8,
        bstream: &mut BinaryStream,
        addr: SocketAddr,
        read_bytes: usize,
    ) -> Result<()> {
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
                let request2 = bstream.decode::<SecondOpenConnectionRequest>().unwrap();
                let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                socket.send(&reply2, addr).await?;
                log::info!("Create new session for {}!", addr);

                let (connected_tx, connected_rx) = mpsc::channel(2048);
                let session = Session::new(addr, connected_tx, socket.clone());
                sessions.lock().await.insert(addr, session);

                // notify about new connection
                sender.send((addr, connected_rx)).await.unwrap();
            }
            _ => {
                log::error!(
                    "Unimplemented packet: 0x{:02X}\nRead data:\n{}",
                    packet_id,
                    Self::bin_to_hex_table(&bstream.get_raw()[..read_bytes])
                );
            }
        }

        Ok(())
    }
}
