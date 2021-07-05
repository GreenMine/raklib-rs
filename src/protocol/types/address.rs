//FIXME: Deprecated
/*#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum IpVersion {
    V4 = 4,
    V6 = 6
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Address {
    pub ip_version: IpVersion,
    pub ip: [u8; 4],
    pub port: u16
}

impl Address {
    pub fn new(ip_version: IpVersion, ip: &str, port: u16) -> Self {
        Self {
            ip_version,
            ip: Self::string_to_ip(ip),
            port
        }
    }

    fn string_to_ip(ip: &str) -> [u8; 4] {
        let mut array = [0u8; 4];
        ip.split('.').enumerate().take_while(|(i, _)| *i < 4).for_each(|(i, octet)| {array[i] = octet.parse::<u8>().unwrap()});

        array
    }
}*/
