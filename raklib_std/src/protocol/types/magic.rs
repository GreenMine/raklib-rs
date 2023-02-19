use std::fmt::{self, Display, Formatter};

use crate::stream::{Adapter, BinaryStream, Result};

pub const MAGIC: Magic = Magic {
    data: [
        0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
        0x78,
    ],
};

#[derive(Clone, Copy, Debug)]
pub struct Magic {
    pub data: [u8; 16],
}

impl Adapter for Magic {
    fn read(bs: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(unsafe { *(bs.read_slice(16)?.as_ptr() as *const Magic) })
    }

    fn add(this: Self, bs: &mut BinaryStream)
    where
        Self: Sized,
    {
        bs.add_slice(&this.data)
    }
}

impl Magic {
    pub fn is_valid(&self) -> bool {
        self.data
            .iter()
            .zip(MAGIC.data.iter())
            .all(|(&a, &b)| a == b)
    }
}

impl Display for Magic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0x")?;
        self.data.iter().try_for_each(|b| write!(f, "{:02x}", b))?;

        Ok(())
    }
}
