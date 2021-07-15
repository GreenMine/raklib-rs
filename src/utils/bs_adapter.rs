use super::BinaryStream;

pub trait BSAdapter: Copy {
    fn read(bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        let res = bs.read_slice_be(std::mem::size_of::<Self>());
        unsafe { *(res.as_ptr() as *const Self) }
    }

    fn add(mut this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(
                (&mut this as *mut Self) as *mut u8,
                std::mem::size_of::<Self>(),
            );
            slice.reverse();
            bs.add_slice(slice)
        }
    }
}

impl BSAdapter for u8 {}
impl BSAdapter for u16 {}
impl BSAdapter for u32 {}
impl BSAdapter for u64 {}
impl BSAdapter for i64 {}
impl BSAdapter for bool {}
