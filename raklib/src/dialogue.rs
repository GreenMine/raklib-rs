use std::{net::SocketAddr, sync::Arc};

use raklib_std::{protocol::packets::Datagram, stream::BinaryStream};

use crate::{
    net::UdpSocket,
    server::{Server, Sessions},
};

// Идея в том, чтобы описать поведение, в котором должны общаться клиент и сервер(в данном случае
// эту структуру буду использовать оба). Т.е. нужно сделать этот класс таким, чтобы его можно было
// использовать как для стабилиации многих соединений(в случае сервер), так и для стабилизации
// единичного соединения(в случае клиента).
//
// 1. Должна пройти стабилизация общения(establish). На данном этапе происходит начальный диалог,
//    который нужен для стабилизации. При стабилиации мы не учитываем порядок пришедший пакетов.
// 2. После стабилизации мы должны
//
//   По сути, после стабилизации соединения, у нас так же будет иметься разделение многое к одному
//   или один к одному(сервер и клиент). Поэтому, у нас должна быть одна настройка, которая
//   позволит менять поведение
//

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] raklib_std::stream::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait DialogueHandler {
    type SessionRef<'a>: std::ops::Deref<Target = crate::session::Session> + std::ops::DerefMut
    where
        Self: 'a;

    fn get_session(&self, addr: SocketAddr) -> Option<Self::SessionRef<'_>>;
    async fn tick(&self);
    async fn unconnected(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet_id: u8,
        bstream: &mut raklib_std::stream::BinaryStream,
    ) -> crate::dialogue::Result<()>;
    fn on_packet();
}

pub struct Dialogue<T: DialogueHandler> {
    handler: T,
    socket: UdpSocket,
}

impl<'a, T: DialogueHandler + Send + Sync + 'a> Dialogue<T> {
    pub fn new(handler: T, socket: UdpSocket) -> Self {
        Self { handler, socket }
    }

    pub async fn run(self)
    where
        <T as DialogueHandler>::SessionRef<'a>: Send + Sync,
    {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));
        let mut bstream = BinaryStream::with_len(2048);

        loop {
            tokio::select! {
                Ok((read_bytes, addr)) = self.socket.recv_from(bstream.get_raw_mut()) => {
                    // TODO: got out of unsafe(may using in bstream, but not there)
                    // SAFETY: u8 doesn't need to drop
                    unsafe { bstream.get_raw_mut().set_len(read_bytes); }

                    log::debug!("Got {} bytes.", read_bytes);

                    let packet_id = bstream.read::<u8>().unwrap();
                    if packet_id & Datagram::BITFLAG_VALID != 0 {
                        if let Some(session) = self.handler.get_session(addr).as_deref_mut() {
                            if packet_id & Datagram::BITFLAG_ACK != 0 {
                                session.handle_ack(bstream.decode().unwrap());
                            } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                                unimplemented!("not acknowledge packet!")
                            } else {
                                session.handle_datagram(bstream.decode().unwrap()).await;
                            }
                        }
                    } else {
                        if let Err(e) = self.handler.unconnected(&self.socket, addr, packet_id, &mut bstream).await {
                            log::error!("Error while handling unconnected message: {}", e);
                        }
                    }

                    bstream.clear();
                }

                _ = interval.tick() => {
                    self.handler.tick().await;
                }
            }
        }
    }
}

pub async fn dialogue_init_server() {
    //
    // Handler::new::<T: Handler>(UdpSocket)
    //          .state(|addr| self.sessions.get(addr).state)
    //          .unconnected(|p| p::ID == Ping { Ok(()) } else { Err(UndefinedPacket) })
    //          .establish(|e| {
    //              e.send(OpenConnectionRequest);
    //              e.wait(OpenConnectionReply);
    //
    //              -validate packet-
    //
    //              e.send(...);
    //              e.wait(...);
    //              ...
    //
    //              Ok(())
    //          })
    //          .handle(|packet| got connected packet)
    //
}
