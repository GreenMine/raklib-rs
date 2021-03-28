use std::fmt::{Display, Formatter, self};
use crate::protocol::consts::MAGIC;

#[derive(Clone, Copy, Debug)]
pub struct Magic {
    pub data: [u8; 16]
}

impl Magic {
    pub fn is_valid(&self) -> bool {
        self.data.iter().zip(MAGIC.data.iter()).all(|(&a, &b)| a == b)
    }
}

impl Display for Magic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0x")?;
        self.data.iter().for_each(|&b| {write!(f, "{:02x}", b).unwrap();});

        Ok(())
    }
}