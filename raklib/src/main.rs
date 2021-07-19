#![feature(macro_attributes_in_derive_output)]

pub mod protocol;
pub mod server;

use server::Server;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:19135".parse().unwrap();

    let mut server = Server::new(address)?;
    server.run()?;

    Ok(())
}
