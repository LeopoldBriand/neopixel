#[repr(C)]
pub struct TxBuf<const N: usize> {
    pub bytes: [[u8; 3]; N],
    pub pad: u8,
}

#[repr(C)]
pub struct TxConfig {
    pub data: *const u8,
    pub len: u16,
    pub port: *mut u8,
    pub hi: u8,
    pub lo: u8,
}
