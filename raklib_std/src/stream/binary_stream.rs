use crate::packet::PacketDecode;
use crate::stream::Error;

use super::{Adapter, Result};

#[derive(Debug)]
pub struct BinaryStream {
    pub(crate) data: Vec<u8>, //TODO: Rewrite it to Box<[u8]>(for more information: https://users.rust-lang.org/t/why-does-putting-an-array-in-a-box-cause-stack-overflow/36493/7)
    pub p: usize,             //FIXME: delete PUB
}

//TODO: Always converting from big-endian to little-endian and vice versa for reading and sending

//New
impl BinaryStream {
    pub fn with_len(len: usize) -> Self {
        Self::new(vec![0u8; len])
    }

    pub fn new(vec: Vec<u8>) -> Self {
        Self { data: vec, p: 0 }
    }
}

//Setters
impl BinaryStream {
    // FIXME: provide data as reference
    pub fn add<T: Adapter>(&mut self, data: T) {
        Adapter::add(&data, self)
    }

    /// There is no need to check for overflow,
    /// because if I record something superfluous, it's the server's problem, not the client's.
    /// Therefore, panic will be an adequate solution
    pub fn add_slice(&mut self, slice: &[u8]) {
        self.data[self.p..self.p + slice.len()].copy_from_slice(slice);
        self.p += slice.len();
    }

    pub fn add_slice_be(&mut self, slice: &[u8]) {
        let dst = &mut self.data[self.p..self.p + slice.len()];
        dst.copy_from_slice(slice);
        dst.reverse();

        self.p += slice.len();
    }
}

//Getters
impl BinaryStream {
    pub fn read<T: Adapter>(&mut self) -> Result<T> {
        T::read(self)
    }

    pub fn read_slice_be(&mut self, n: usize) -> Result<&[u8]> {
        let res = self.read_slice(n)?;
        res.reverse();

        Ok(res)
    }

    //FIXME: Check the overflow
    pub fn read_slice(&mut self, n: usize) -> Result<&mut [u8]> {
        if self.p + n > self.data.len() {
            return Err(Error::EndOfStream {
                try_to_read: n,
                actual_left: self.data.len() - self.p,
            });
        }

        let result = &mut self.data[self.p..self.p + n];
        self.p += n;

        Ok(result)
    }
}

//Misc
impl BinaryStream {
    //TODO: -> Result<()>
    pub fn skip(&mut self, n: usize) {
        self.p += n;
    }
    pub fn clear(&mut self) {
        self.p = 0;
        self.data.resize(self.data.capacity(), 0u8);
    }

    pub fn decode<T: PacketDecode>(&mut self) -> Result<T> {
        T::decode(self)
    }

    pub fn is_end(&self) -> bool {
        self.p == self.data.len()
    }

    pub fn get_offset(&self) -> usize {
        self.p
    }

    pub fn get_data(self) -> Vec<u8> {
        self.data
    }

    //TODO: maybe, random access is not logical for stream
    pub fn get_raw(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn get_raw_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}
