use std::fmt::{Display, Formatter, self};
#[derive(Clone, Copy)]
pub struct Magic {
    pub data: [u8; 16]
}

impl Display for Magic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0x")?;
        self.data.iter().for_each(|&b| {write!(f, "{:02x}", b).unwrap();});

        Ok(())
    }
}