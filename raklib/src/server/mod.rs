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

pub type ConnectedData = (
    tokio::sync::mpsc::Receiver<raklib_std::stream::BinaryStream>,
    std::net::SocketAddr,
);
