use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::utils::BSAdapter;

impl BSAdapter for SocketAddr {
    fn add(this: Self, bs: &mut crate::utils::BinaryStream)
    where
        Self: Sized,
    {
        bs.add(if this.is_ipv4() { 4u8 } else { 6u8 });

        bs.add_slice(&match this.ip() {
            IpAddr::V4(addr) => addr.octets(),
            IpAddr::V6(_addr) => unimplemented!(),
        });

        bs.add(this.port());
        //from raw parts...............
    }

    //FIXME: only IPv4
    fn read(bs: &mut crate::utils::BinaryStream) -> Self
    where
        Self: Sized,
    {
        bs.skip(1);
        SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(bs.read(), bs.read(), bs.read(), bs.read())),
            bs.read(),
        )
    }
}
