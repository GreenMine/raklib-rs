use super::types::Magic;

pub const MAGIC: Magic = Magic {
    data: [
        0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
        0x78,
    ],
};

pub const SERVER_GUID: u64 = 0x23ace8d3829791d6;

pub const TICKS: u8 = 20;
pub const TIME_PER_TICK: u128 = (1000.0 / (TICKS as f32)) as u128;

pub const SERVER_TITLE: &str =
    "MCPE;Rust core test!;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;";
pub const PROTOCOL_VERSION: u8 = 11;
