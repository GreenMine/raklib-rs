use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, unimplemented};

use crate::protocol::{packets::PacketDecode, types::{Magic, RakNetString, U24}};


pub struct BinaryStream {
    pub data: Vec<u8>, //TODO: Rewrite it to Box<[u8]>(for more information: https://users.rust-lang.org/t/why-does-putting-an-array-in-a-box-cause-stack-overflow/36493/7)
    p: usize
}

//TODO: Always converted from big-endian to little-endian and vice versa for reading and sending 

//New
impl BinaryStream {
    pub fn with_len(len: usize) -> Self {
        Self::new(vec![0u8; len])
    }

    pub fn new(vec: Vec<u8>) -> Self {
        Self {data: vec, p : 0}
    }
}

//Setters
impl BinaryStream {
    pub fn add<T>(&mut self, mut data: T) {
        unsafe {
            let slice = std::slice::from_raw_parts_mut((&mut data as *mut T) as *mut u8, std::mem::size_of::<T>());
            slice.reverse();
            self.add_slice(slice)
        }
    }

    //FIXME: Check the overflow
    pub fn add_slice(&mut self, slice: &[u8]) {
        &self.data[self.p..self.p + slice.len()].copy_from_slice(slice);
        self.p += slice.len();
    }
}

//Getters
impl BinaryStream {
    pub fn read<T: Copy>(&mut self) -> T {
        let res = self.read_slice_be(std::mem::size_of::<T>());
        
        unsafe {
            *(res.as_ptr() as *const T)
        }
    }

    pub fn read_slice_be(&mut self, n: usize) -> &[u8] {
        let res = self.read_slice(n);
        res.reverse();

        res
    }

    //FIXME: Check the overflow
    pub fn read_slice(&mut self, n: usize) -> &mut [u8] {
        let result = &mut self.data[self.p..self.p + n];
        self.p += n;

        result
    }
}

//Misc
impl BinaryStream {
    pub fn skip(&mut self, n: usize) {
        self.p += n;
    }
    pub fn clear(&mut self) {
        self.p = 0;
    }

    pub fn decode<T: PacketDecode>(&mut self) -> T {
        T::decode(self)
    }
}

//Env
impl BinaryStream {
    pub fn add_uint24le(&mut self, num: U24) {
        self.add_slice(&num.data[..]);
    }

    pub fn add_string(&mut self, string: &RakNetString) {
        self.add(string.length);
        self.add_slice(string.data);
    }

    pub fn add_magic(&mut self, magic: Magic) {
        self.add_slice(&magic.data[..]);
    }

    pub fn add_address(&mut self, address: SocketAddr) {
        self.add(if address.is_ipv4() { 4u8 } else { 6u8 });

        self.add_slice(&match address.ip() {
            IpAddr::V4(addr) => addr.octets(),
            IpAddr::V6(_addr) => unimplemented!() 
        });

        self.add(address.port());
        //from raw parts...............
    }

    pub fn read_string(&mut self) -> RakNetString {
        let len = self.read();
        RakNetString {
            length: len,
            data: self.read_slice(len as usize)
        }
    }

    pub fn read_magic(&mut self) -> Magic {
        unsafe {*(self.read_slice(16).as_ptr() as *const Magic)}
    }

    //FIXME: only IPv4
    pub fn read_address(&mut self) -> SocketAddr {
        self.skip(1);
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(self.read(), self.read(), self.read(), self.read())), self.read())
    }
}