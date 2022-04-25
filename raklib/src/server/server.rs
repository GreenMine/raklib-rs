use crate::protocol::{consts::TIME_PER_TICK, packets::connected::Datagram};
use crate::*;
use raklib_std::stream::BinaryStream;
use std::sync::Arc;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use super::{Sessions, UdpSocket};

use crate::server::ConnectedData;
use tokio::sync::{mpsc, Mutex};

pub struct Server {
    pub(super) socket: Arc<UdpSocket>, // FIXME: fuck RefCounter
    pub(super) _start_time: Instant,
    pub(super) sessions: Arc<Mutex<Sessions>>,
    pub(super) mpsc: (mpsc::Sender<ConnectedData>, mpsc::Receiver<ConnectedData>),
}

unsafe impl Send for Server {}

impl Server {
    pub async fn bind(address: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            socket: Arc::new(UdpSocket::bind(address).await?),
            _start_time: Instant::now(),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            mpsc: mpsc::channel(100), //FIXME:
        })
    }

    pub async fn recv(&mut self) -> Option<ConnectedData> {
        self.mpsc.1.recv().await
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        log!(
            "RakNet connection opened on {}",
            self.socket.get_bind_address()
        );

        let socket = Arc::clone(&self.socket);
        let sessions = Arc::clone(&self.sessions);
        let sender = self.mpsc.0.clone();

        tokio::spawn(async move {
            let mut bstream = BinaryStream::with_len(2048);
            loop {
                let tick_start = Instant::now();

                //TODO: maybe need to rewrite it(now pasted from pmmp raklib implementation)
                for _ in 0..100 {
                    if let Ok((read_bytes, addr)) = socket.try_recv_from(bstream.get_raw_mut()) {
                        bstream.data.truncate(read_bytes); //FIXME: truncate free truncated elements memory block
                        let packet_id = bstream.read::<u8>().unwrap();

                        if packet_id & Datagram::BITFLAG_VALID != 0 {
                            if let Some(session) = sessions.lock().await.get_mut(&addr) {
                                if packet_id & Datagram::BITFLAG_ACK != 0 {
                                    session.handle_ack(bstream.decode().unwrap());
                                } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                                    unimplemented!("not acknowledge packet!");
                                } else {
                                    session.handle_datagram(bstream.decode().unwrap()).await;
                                }
                            }
                        } else {
                            Server::unconnected_handler(
                                &socket,
                                &sender,
                                &sessions,
                                packet_id,
                                &mut bstream,
                                addr,
                                read_bytes,
                            )
                            .await
                            .unwrap();
                        }

                        bstream.clear();
                    }
                }

                for session in sessions.lock().await.values_mut() {
                    session.update().await;
                } //Update all sessions

                let tick_lead_ms = tick_start.elapsed().as_millis();
                if tick_lead_ms < TIME_PER_TICK {
                    tokio::time::sleep(Duration::from_millis(
                        (TIME_PER_TICK - tick_lead_ms) as u64,
                    ))
                    .await;
                }
            }
        });

        Ok(())
    }

    pub(crate) fn bin_to_hex_table(bin: &[u8]) -> String {
        let mut str = String::new();
        bin.iter().enumerate().for_each(|(i, &b)| {
            str += &format!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                str += "\n";
            }
        });

        str + "\n"
    }
}
