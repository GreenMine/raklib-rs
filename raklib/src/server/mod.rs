mod listener;
mod server;
pub mod session;

pub(super) use listener::Listener;
pub use server::Server;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] raklib_std::stream::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub type Sessions = std::collections::HashMap<std::net::SocketAddr, session::Session>;
pub type ConnectedData = (
    std::net::SocketAddr,
    tokio::sync::mpsc::Receiver<raklib_std::stream::BinaryStream>,
);
