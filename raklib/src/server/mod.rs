mod server;

pub use server::Server;

pub type Sessions = dashmap::DashMap<std::net::SocketAddr, crate::session::Session>;
pub type ConnectedData = (
    std::net::SocketAddr,
    tokio::sync::mpsc::Receiver<raklib_std::stream::BinaryStream>,
);
