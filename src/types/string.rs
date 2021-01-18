pub struct RakNetString<'a> {
    pub length: u16,
    pub data: &'a [u8] 
}

impl<'a> RakNetString<'a> {
    pub fn from_string(string: &'a String) -> Self {
        Self {
            length: string.len() as u16,
            data: string.as_bytes()
        }
    }
}