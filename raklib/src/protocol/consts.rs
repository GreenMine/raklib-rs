use std::time::Duration;

pub const SERVER_GUID: u64 = 0x23ace8d3829791d6;

pub const TICKS: u8 = 20;
pub const TIME_PER_TICK: Duration = Duration::from_millis((1000.0 / (TICKS as f32)) as u64);

pub const SERVER_TITLE: &str =
    "MCPE;Rust core test!;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;";
pub const PROTOCOL_VERSION: u8 = 11;
