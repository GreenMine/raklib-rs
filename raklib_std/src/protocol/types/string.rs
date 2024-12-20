use std::ops::{Index, Range};
use std::str::Utf8Error;

use crate::stream::{Adapter, BinaryStream, Result};

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

impl Adapter for RakNetString<'_> {
    fn read(_bs: &mut BinaryStream) -> Result<Self>
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

    fn add(&self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add(self.length);
        bs.add_slice(self.data);
    }
}

impl Index<usize> for RakNetString<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<Range<usize>> for RakNetString<'_> {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<'a> std::convert::TryInto<&'a str> for RakNetString<'a> {
    type Error = Utf8Error;

    fn try_into(self) -> std::result::Result<&'a str, Self::Error> {
        std::str::from_utf8(self.data)
    }
}

impl Adapter for String {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        let len: u16 = bs.read()?;
        log::info!("Len of string: {}", len);
        let raw_str = bs.read_slice(len as usize)?;
        Ok(String::from_utf8_lossy(raw_str).to_string())
    }

    fn add(&self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add(self.len() as u16);
        bs.add_slice(self.as_bytes())
    }
}
