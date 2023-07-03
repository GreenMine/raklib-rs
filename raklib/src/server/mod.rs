mod server;
mod unconnected_handler;

pub use server::Server;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] raklib_std::stream::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

pub type Sessions = dashmap::DashMap<std::net::SocketAddr, crate::session::Session>;
pub type ConnectedData = (
    std::net::SocketAddr,
    tokio::sync::mpsc::Receiver<raklib_std::stream::BinaryStream>,
);
