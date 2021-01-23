use std::ops::{Index, Range};

pub struct RakNetString<'a> {
    pub length: u16,
    pub data: &'a [u8] 
}

impl<'a> RakNetString<'a> {
    pub fn from_string(string: &'a String) -> Self {
        Self {
            length: string.len() as u16,
            data: string.as_bytes()
        }
    }
}

impl <'a> Index<usize> for RakNetString<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl <'a> Index<Range<usize>> for RakNetString<'a> {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}