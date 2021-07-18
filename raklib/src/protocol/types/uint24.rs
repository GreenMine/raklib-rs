use raklib_std::utils::{BSAdapter, BinaryStream};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct u24 {
    pub data: [u8; 3],
}

impl BSAdapter for u24 {
    fn read(bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        unsafe { *(bs.read_slice(3).as_ptr() as *const u24) }
    }

    fn add(this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add_slice(&this.data[..]);
    }
}

impl From<u32> for u24 {
    fn from(data: u32) -> Self {
        assert!(data <= 0xFFFFFF, "overflow when convert u32 to u24");

        let mut array = [0u8; 3];
        let result = unsafe { std::slice::from_raw_parts((&data as *const u32) as *const u8, 4) };
        array.clone_from_slice(&result[..3]);

        Self { data: array }
    }
}

impl From<u24> for u32 {
    fn from(number: u24) -> Self {
        let mut result = [0u8; 4];
        result[0..3].clone_from_slice(&number.data);

        unsafe { *(result.as_ptr() as *const u32) }
    }
}

impl std::fmt::Display for u24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

impl std::fmt::Debug for u24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
