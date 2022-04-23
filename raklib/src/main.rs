#![feature(type_name_of_val)]

pub mod protocol;
pub mod server;
pub mod utils;

pub use utils::log;

use server::Server;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:19135".parse().unwrap();

    let mut server = Server::new(address)?;
    server.run()?;

    Ok(())
}
