mod unconnected;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::time::Instant;

use raklib_std::protocol::packets::Datagram;
use raklib_std::stream::BinaryStream;

use super::Sessions;
use crate::net::UdpSocket;
use crate::protocol::consts::TIME_PER_TICK;
use crate::server::ConnectedData;

pub struct Listener {
    socket: Arc<UdpSocket>, // FIXME: fuck RefCounter
    sessions: Sessions,
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
        let mut bstream = BinaryStream::with_len(2048);
        let mut tick = Instant::now() + TIME_PER_TICK;
        loop {
            tokio::select! {
                Ok((read_bytes, addr)) = self.socket.recv_from(bstream.get_raw_mut()) => {
                    // TODO: got out of unsafe(may using in bstream, but not there)
                    // SAFETY: u8 doesn't need to drop
                    unsafe {
                        bstream.get_raw_mut().set_len(read_bytes);
                    }

                    let packet_id = bstream.read::<u8>().unwrap();

                    if packet_id & Datagram::BITFLAG_VALID != 0 {
                        if let Some(session) = self.sessions.get_mut(&addr) {
                            if packet_id & Datagram::BITFLAG_ACK != 0 {
                                session.handle_ack(bstream.decode().unwrap());
                            } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                                unimplemented!("not acknowledge packet!");
                            } else {
                                session.handle_datagram(bstream.decode().unwrap()).await;
                            }
                        }
                    } else {
                        self.handle_unconnected(
                            packet_id,
                            &mut bstream,
                            addr,
                            read_bytes,
                        ).await.unwrap();
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
        });
    }
}
