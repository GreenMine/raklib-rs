mod session;
pub type Sessions = std::collections::HashMap<std::net::SocketAddr, Session>;

mod handle_error;
mod server;
mod udp_socket;
mod unconnected_handler;

pub use handle_error::HandleError;
pub use server::Server;
pub use session::Session;
pub(crate) use udp_socket::UdpSocket;

pub type Result<T> = std::result::Result<T, HandleError>;
pub type ConnectedData = (std::net::SocketAddr, tokio::sync::mpsc::Receiver<Vec<u8>>);
