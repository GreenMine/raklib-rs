use crate::packet::PacketDecode;

use super::BSAdapter;

pub struct BinaryStream {
    pub data: Vec<u8>, //TODO: Rewrite it to Box<[u8]>(for more information: https://users.rust-lang.org/t/why-does-putting-an-array-in-a-box-cause-stack-overflow/36493/7)
    pub p: usize,      //FIXME: delete PUB
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
    pub fn add<T: BSAdapter>(&mut self, data: T) {
        BSAdapter::add(data, self);
    }

    //FIXME: Check the overflow
    pub fn add_slice(&mut self, slice: &[u8]) {
        self.data[self.p..self.p + slice.len()].copy_from_slice(slice);
        self.p += slice.len();
    }
}

//Getters
impl BinaryStream {
    pub fn read<T: BSAdapter>(&mut self) -> T {
        T::read(self)
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
        self.data.resize(self.data.capacity(), 0u8);
    }

    pub fn decode<T: PacketDecode>(&mut self) -> T {
        let res = T::decode(self);

        res
    }

    pub fn is_end(&self) -> bool {
        self.p == self.data.len()
    }

    pub fn get_raw(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn get_raw_mut(&mut self) -> &mut [u8] {
        &mut self.data[..]
    }
}
