use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::BinaryStream;
use super::Result;

pub trait Adapter: Clone {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized;

    fn add(&self, bs: &mut BinaryStream)
    where
        Self: Sized;
}

macro_rules! impl_for_base_type {
    ( $($t:ty),* ) => {
    $( impl Adapter for $t {
        fn read(bs: &mut BinaryStream) -> Result<Self>
        where
            Self: Sized,
        {
            let res = bs.read_slice_be(std::mem::size_of::<Self>())?;
            Ok(unsafe { (*(res.as_ptr() as *const Self)).clone() }) //TODO: later fix it.
        }

        fn add(&self, bs: &mut BinaryStream)
        where
            Self: Sized,
        {
            unsafe {
                let slice = std::slice::from_raw_parts(
                    (self as *const Self) as *const u8,
                    std::mem::size_of::<Self>(),
                );
                bs.add_slice_be(slice)
            }
        }
    }) *
    }
}

impl_for_base_type! { u8, u16, u32, u64, i16, i32, i64, bool }

impl Adapter for SocketAddr {
    //FIXME: only IPv4
    fn read(bs: &mut crate::stream::BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        bs.skip(1);
        Ok(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                bs.read::<u8>()?,
                bs.read()?,
                bs.read()?,
                bs.read()?,
            )),
            bs.read()?,
        ))
    }

    fn add(&self, bs: &mut crate::stream::BinaryStream)
    where
        Self: Sized,
    {
        bs.add(if self.is_ipv4() { 4u8 } else { 6u8 });

        bs.add_slice(&match self.ip() {
            IpAddr::V4(addr) => addr.octets(),
            IpAddr::V6(_addr) => unimplemented!(),
        });

        bs.add(self.port());
        //from raw parts...............
    }
}

impl<T: Adapter> Adapter for &T {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn add(&self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        T::add(self, bs)
    }
}
