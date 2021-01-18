pub struct RakNetString {
    length: u16,
    data: Box<[u8]>
}

impl RakNetString {
    pub fn from_string(string: String) -> Self {
        Self {
            length: string.len() as u16,
            data: string.as_bytes().into()
        }
    }
}