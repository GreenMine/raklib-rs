use std::ops::{Index, Range};

use raklib_std::utils::{BSAdapter, BinaryStream};

#[derive(Clone, Copy)]
pub struct RakNetString<'a> {
    pub length: u16,
    pub data: &'a [u8],
}

impl<'a> RakNetString<'a> {
    pub fn from_string(string: &'a str) -> Self {
        assert!(string.is_ascii()); //FIXME
        Self {
            length: string.len() as u16,
            data: string.as_bytes(),
        }
    }
}

impl<'a> BSAdapter for RakNetString<'a> {
    fn read(_bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        /*let str_len: u16 = bs.read();
        Self {
            length: str_len,
            data: bs.read_slice(str_len as usize),
        }*/
        unimplemented!("read RakNet string")
    }

    fn add(this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add(this.length);
        bs.add_slice(this.data);
    }
}

impl<'a> Index<usize> for RakNetString<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a> Index<Range<usize>> for RakNetString<'a> {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}
