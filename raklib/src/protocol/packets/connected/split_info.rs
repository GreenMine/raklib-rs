use raklib_std::stream::{Adapter, BinaryStream, Result};

#[derive(Copy, Clone, Debug)]
pub struct SplitInfo {
    pub fragment_id: i16,
    pub fragment_amount: i32,
    pub fragment_index: i32,
}

impl Adapter for SplitInfo {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(SplitInfo {
            fragment_amount: bs.read()?,
            fragment_id: bs.read()?,
            fragment_index: bs.read()?,
        })
    }

    fn add(_this: Self, _bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        todo!()
    }
}
