use std::net::UdpSocket;


mod utils;
mod types;

use utils::BinaryStream;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:19133")?;
    let mut buffer = [0u8; 4096];
    
    println!("Waiting message...");
    loop {
        let (readed_bytes, addr) = socket.recv_from(&mut buffer).expect("no date received!");
        let start = std::time::Instant::now();

        /*print!("Result: '");
        &buffer[0..readed_bytes].into_iter().for_each(|&d| print!("{}", d as char));
        println!("'");*/
        /*println!("Got:");
        print_binary(&buffer[0..readed_bytes]);*/

        let mut bstream = BinaryStream::from_slice(&buffer[..readed_bytes]);
        let packet_id = bstream.read::<u8>();
        match packet_id {
            0x1 => {
                let time = bstream.read::<u64>();
                let magic = bstream.read_magic();
                let client_guid = bstream.read::<u64>();

                /*println!("Time: {}", time);
                println!("Magic: {}", magic);
                println!("Client GUID: {}", client_guid);*/

                let server_id_string = "MCPE;Rust core test;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;";
                let server_guid: u64 = 0x23ace8d3829791d6;

                let server_id_array = server_id_string.chars().map(|c| c as u8).collect::<Vec<u8>>();

                let mut response: BinaryStream = BinaryStream::with_len(1 + 8 + 8 + 16 + (2 + server_id_string.len()));
                response.set(0x1c_u8);
                response.set(time);
                response.set(server_guid);
                response.set_magic(magic);
                response.set(server_id_array.len() as u16);
                response.set_slice(&server_id_array[..]);

                /*println!("Sended:");
                print_binary(&response.data[..]);*/

                socket.send_to(&response.data[..], addr)?;
            },
            0x5 => {
                println!("Result");
                print_binary(bstream.read_slice(readed_bytes - 1));
            }
            _ => {
                unimplemented!("PACKET ID: 0x{:02X}", packet_id)
            }
        }

        println!("One packet execution time: {}ms.", start.elapsed().as_millis());
    }
    Ok(())
}

fn print_binary(bin: &[u8]) {
        bin.iter().enumerate().for_each(|(i, &b)| {
            print!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                println!();
            }
        });
        println!();
}