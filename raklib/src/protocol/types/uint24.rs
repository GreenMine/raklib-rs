use raklib_std::utils::{BSAdapter, BinaryStream};
use std::ops::{Add, AddAssign};

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

impl BSAdapter for u24 {
    fn read(bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        unsafe {
            let mut num = *(bs.read_slice(3).as_ptr() as *const u32);
            num &= 0x00FFFFFF; //xD

            Self { num }
        }
    }

    fn add(this: Self, bs: &mut BinaryStream) -> raklib_std::utils::Result<()>
    where
        Self: Sized,
    {
        unsafe {
            bs.add_slice(std::slice::from_raw_parts(
                (&this.num as *const u32) as *const u8,
                3,
            ))
        }
    }
}

impl From<u32> for u24 {
    fn from(num: u32) -> Self {
        assert!(!(num > 0xFFFFFF), "overflow when convert u32 to u24");

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
