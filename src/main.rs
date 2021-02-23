mod utils;
mod types;
mod consts;
mod packets;
mod server;

use server::UdpServer;


fn main() -> std::io::Result<()> {
    let address = "127.0.0.2:19155";

    let mut server = UdpServer::new(address)?;
    server.run()?;

    Ok(())
}