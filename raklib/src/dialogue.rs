use std::{net::SocketAddr, collections::HashMap};

use raklib_std::{stream::BinaryStream, protocol::packets::Datagram};

use crate::{server::session::{Status}, net::UdpSocket};




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

pub trait DialogueHandler {
    type Session: Handshake + Receipts;

    fn get_session(&self, addr: SocketAddr) -> Self::Session;
    fn unconnected(&self, packet: BinaryStream) -> Result<(), ()>;
    fn establish();
    fn on_packet();
}

pub struct Dialogue<T: DialogueHandler> {
    handler: T,
    socket: UdpSocket
}

impl<T: DialogueHandler + Send + 'static> Dialogue<T> {
    pub fn new(handler: T, socket: UdpSocket) -> Self {
        Self { handler, socket }
    }

    pub async fn run(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));
            let mut bstream = BinaryStream::with_len(2048);

            loop {
                if let Ok((read_bytes, addr)) = self.socket.try_recv_from(bstream.get_raw_mut()) {
                    // TODO: got out of unsafe(may using in bstream, but not there)
                    // SAFETY: u8 doesn't need to drop
                    unsafe { bstream.get_raw_mut().set_len(read_bytes); }

                    let packet_id = bstream.read::<u8>().unwrap();
                    if packet_id & Datagram::BITFLAG_VALID != 0 {
                    } else {
                    }

                    bstream.clear();

                }
                
                interval.tick().await;
            }
        });
    }
}

pub trait Handshake {}

pub trait Receipts {
    fn handle_ack();
    fn handle_nack();
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