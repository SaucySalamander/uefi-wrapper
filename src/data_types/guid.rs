#[derive(Debug)]
#[repr(C)]
pub struct Guid {
    time_low: u32,
    time_mid: [u8; 2],
    time_high_and_version: [u8; 2],
    clock_seq_hi_and_reserved: u8,
    clock_seq_low: u8,
    node: [u8; 6],
}

impl Guid {}
