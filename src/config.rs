//! VM configuration parameters
#![allow(dead_code)]

/// max VM memory size
pub const MSZ: usize = 0x10000;
/// return stack size (max call depth)
pub const RSZ: usize = 0x100;
/// data stack size (limited)
pub const DSZ: usize = 0x10;

/// Web server default bind ip
pub const IP: &str = "127.0.0.1";
// pub const IP: &str = "0.0.0.0";
/// Web server IP port
pub const PORT: u16 = 12345;
/// bind address constant
pub const BIND: &str = const_format::formatcp!("{IP}:{PORT}");
