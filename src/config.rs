//! VM conficuration parameters
#![allow(dead_code)]

/// max VM memory size
pub const MSZ: usize = 0x10000;
/// return stack size (max call depth)
pub const RSZ: usize = 0x100;
/// data stack size (limited)
pub const DSZ: usize = 0x10;

pub const IP: &str = "127.0.0.1";
pub const PORT: u16 = 12345;
