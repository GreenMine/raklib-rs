use raklib_std::stream::{BSAdapter, BinaryStream};

#[derive(Copy, Clone, Debug)]
pub struct SplitInfo {
    pub fragment_id: i16,
    pub fragment_amount: i32,
    pub fragment_index: i32,
}

impl BSAdapter for SplitInfo {
    fn read(bs: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        SplitInfo {
            fragment_amount: bs.read(),
            fragment_id: bs.read(),
            fragment_index: bs.read(),
        }
    }

    fn add(this: Self, bs: &mut BinaryStream) -> raklib_std::stream::Result<()>
    where
        Self: Sized,
    {
        todo!()
    }
}
