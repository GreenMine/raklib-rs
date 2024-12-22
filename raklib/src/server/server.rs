use std::net::SocketAddr;

use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::net::{Error as NetError, UdpSocket};
use crate::server::ConnectedData;

use super::Listener;

pub struct Server {
    pub(super) _start_time: Instant,
    pub(super) receiver: mpsc::Receiver<ConnectedData>,
}

impl Server {
    pub async fn bind(address: SocketAddr) -> Result<Self, NetError> {
        let socket = UdpSocket::bind(address).await?;
        let (tx, rx) = mpsc::channel(100); // FIXME:

        let server = Self {
            _start_time: Instant::now(),
            receiver: rx,
        };

        let listener = Listener::new(socket, tx);
        tokio::spawn(async move { listener.listen().await });

        Ok(server)
    }

    pub async fn accept(&mut self) -> Option<ConnectedData> {
        self.receiver.recv().await
    }
}
