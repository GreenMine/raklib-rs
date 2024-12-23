use std::ops::{Add, AddAssign};

use crate::stream::{Adapter, BinaryStream, Result};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct u24 {
    pub num: u32,
}

impl u24 {
    pub fn inc(&mut self) {
        *self += u24::from(1);
    }
}

impl Adapter for u24 {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        let bytes = bs.read_slice(3)?;
        let num = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]);

        Ok(Self { num })
    }

    fn add(&self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add_slice(&self.num.to_le_bytes()[..3])
    }
}

impl From<u32> for u24 {
    fn from(num: u32) -> Self {
        assert!(num <= 0xFFFFFF, "overflow when convert u32 to u24");

        Self { num }
    }
}

impl From<u24> for u32 {
    fn from(number: u24) -> Self {
        number.num
    }
}

impl Add for u24 {
    type Output = u24;

    fn add(self, rhs: Self) -> Self::Output {
        u24::from(self.num + rhs.num)
    }
}

impl AddAssign for u24 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::fmt::Display for u24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl std::fmt::Debug for u24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
