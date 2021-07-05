#[derive(Clone, Copy, Debug)]
pub struct U24 {
    pub data: [u8; 3],
}

impl From<u32> for U24 {
    fn from(data: u32) -> Self {
        assert!(data <= 0xFFFFFF, "overflow when convert u32 to u24");

        let mut array = [0u8; 3];
        let result = unsafe { std::slice::from_raw_parts((&data as *const u32) as *const u8, 4) };
        array.clone_from_slice(&result[..3]);

        Self { data: array }
    }
}
