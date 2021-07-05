use RakLib::server::UdpServer;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:19132";

    let mut server = UdpServer::new(address)?;
    server.run()?;

    Ok(())
}
