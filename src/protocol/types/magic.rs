use crate::protocol::consts::MAGIC;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct Magic {
    pub data: [u8; 16],
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
        self.data
            .iter()
            .map(|&b| write!(f, "{:02x}", b))
            .collect::<Result<_, _>>()?;

        Ok(())
    }
}
