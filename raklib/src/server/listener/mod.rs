mod unconnected;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::time::Instant;

use raklib_std::protocol::packets::Datagram;
use raklib_std::stream::BinaryStream;

use crate::net::UdpSocket;
use crate::protocol::consts::TIME_PER_TICK;
use crate::server::ConnectedData;

use super::Result;

pub struct Listener {
    socket: Arc<UdpSocket>, // FIXME: fuck RefCounter
    sessions: HashMap<std::net::SocketAddr, crate::server::session::Session>,
    sender: mpsc::Sender<ConnectedData>,
}

impl Listener {
    pub fn new(socket: UdpSocket, sender: mpsc::Sender<ConnectedData>) -> Self {
        Self {
            socket: Arc::new(socket),
            sessions: HashMap::new(),
            sender,
        }
    }

    pub async fn listen(mut self) {
        tracing::info!(
            "RakNet listener started at {:?}",
            self.socket.get_bind_address()
        );

        let mut bstream = BinaryStream::with_len(2048);
        let mut tick = Instant::now() + TIME_PER_TICK;
        loop {
            tokio::select! {
                Ok((received_bytes, addr)) = self.socket.recv_from(bstream.get_raw_mut()) => {
                    // TODO: got out of unsafe(may using in bstream, but not there)
                    // SAFETY: u8 doesn't need to drop
                    unsafe {
                        bstream.get_raw_mut().set_len(received_bytes);
                    }

                    let span = tracing::info_span!("handle", ?addr);
                    let _guard = span.enter();

                    match self.handle_packet(&mut bstream, addr).await {
                        Ok(_) => {},
                        Err(e) => tracing::error!(?e, "handling packet")
                    }

                    bstream.clear();
                },
                _ = tokio::time::sleep_until(tick) => {
                    self.update_sessions().await;
                    tick = Instant::now() + TIME_PER_TICK;
                }
            }
        }
    }

    async fn handle_packet(&mut self, packet: &mut BinaryStream, addr: SocketAddr) -> Result<()> {
        let packet_id = packet.read::<u8>()?;

        if packet_id & Datagram::BITFLAG_VALID != 0 {
            if let Some(session) = self.sessions.get_mut(&addr) {
                if packet_id & Datagram::BITFLAG_ACK != 0 {
                    session.handle_ack(packet.decode()?);
                } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                    unimplemented!("not acknowledge packet!");
                } else {
                    session.handle_datagram(packet.decode()?).await;
                }
            }
        } else {
            self.handle_unconnected(packet_id, packet, addr).await?;
        }

        Ok(())
    }

    // TODO: using retain for removing or something like that
    async fn update_sessions(&mut self) {
        let mut need_to_remove = Vec::new();

        for session in self.sessions.values_mut() {
            session.update().await;

            if !session.status.is_connected() {
                need_to_remove.push(session.get_addr());
            }
        }

        need_to_remove.iter().for_each(|a| {
            self.sessions.remove(a);
            tracing::trace!(address = ?a, "session removed");
        });
    }
}
