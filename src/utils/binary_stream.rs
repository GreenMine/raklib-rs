use crate::types::Magic;

pub struct BinaryStream {
    pub data: Vec<u8>,
    p: usize
}

//TODO: Always converted from big-endian to little-endian and vice versa for reading and sending 

//New
impl BinaryStream {
    pub fn from_slice(slice: &[u8]) -> Self {
        Self::new(slice.to_vec())
    }

    pub fn with_len(len: usize) -> Self {
        Self::new(vec![0u8; len])
    }

    pub fn new(vec: Vec<u8>) -> Self {
        Self {data: vec, p : 0}
    }

}

//Setters
impl BinaryStream {
    pub fn set<T>(&mut self, mut data: T) {
        unsafe {
            let slice = std::slice::from_raw_parts_mut((&mut data as *mut T) as *mut u8, std::mem::size_of::<T>());
            slice.reverse();
            self.set_slice(slice)
        }
    }

    pub fn set_magic(&mut self, magic: Magic) {
        self.set_slice(&magic.data[..]);
    }

    //FIXME: Check the overflow
    pub fn set_slice(&mut self, slice: &[u8]) {
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

    pub fn read_magic(&mut self) -> Magic {
        unsafe {*(self.read_slice(16).as_ptr() as *const Magic)}
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
}