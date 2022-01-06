use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::BinaryStream;

pub trait BSAdapter: Clone {
    fn read(bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        let res = bs.read_slice_be(std::mem::size_of::<Self>());
        unsafe { (*(res.as_ptr() as *const Self)).clone() } //TODO: later fix it.
    }

    fn add(mut this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(
                (&mut this as *mut Self) as *mut u8,
                std::mem::size_of::<Self>(),
            );
            slice.reverse();
            bs.add_slice(slice)
        }
    }
}

macro_rules! impl_for_base_type {
    ( $($t:ty),* ) => {
    $( impl BSAdapter for $t {}) *
    }
}

impl_for_base_type! { u8, u16, u32, u64, i16, i32, i64, bool }

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

/*impl<T: BSAdapter> BSAdapter for Vec<T> {
    fn add(this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        this.into_iter().for_each(|p| bs.add(p));
    }

    fn read(_bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        unimplemented!("read operation for Vec<T>")
    }
}*/
