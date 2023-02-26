pub use handle_error::HandleError;
pub use server::Server;

pub type Sessions = std::collections::HashMap<std::net::SocketAddr, session::Session>;
mod handle_error;
mod server;
pub mod session;
mod unconnected_handler;

pub type Result<T> = std::result::Result<T, HandleError>;
pub type ConnectedData = (
    std::net::SocketAddr,
    tokio::sync::mpsc::Receiver<raklib_std::stream::BinaryStream>,
);
