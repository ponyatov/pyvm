//! VM configuration parameters
#![allow(dead_code)]
// #![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/// max VM memory size
pub const VM_MSZ: usize = 0x10000;
/// return stack size (max call depth)
pub const VM_RSZ: usize = 0x100;
/// data stack size (limited)
pub const VM_DSZ: usize = 0x10;

/// Web server default bind ip
pub mod server {
    pub const ip: &str = "127.0.0.1";
    // pub const IP: &str = "0.0.0.0";
    /// Web server IP port
    pub const port: u16 = 12345;
    /// bind address constant
    pub const bind: &str = const_format::formatcp!("{ip}:{port}");
}

/// screen width
pub mod gui {
    pub const width: u16 = 240;
    pub const height: u16 = 320;
    pub const icon_size: u16 = 64;
}
