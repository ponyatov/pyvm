//! VM configuration parameters
#![allow(dead_code)]

/// max VM memory size
pub const VM_MSZ: usize = 0x10000;
/// return stack size (max call depth)
pub const VM_RSZ: usize = 0x100;
/// data stack size (limited)
pub const VM_DSZ: usize = 0x10;

/// Web server default bind ip
pub const SERVER_IP: &str = "127.0.0.1";
// pub const IP: &str = "0.0.0.0";
/// Web server IP port
pub const SERVER_PORT: u16 = 12345;
/// bind address constant
pub const SERVER_BIND: &str = const_format::formatcp!("{SERVER_IP}:{SERVER_PORT}");

/// screen width
pub const SCREEN_WIDTH: u16 = 240;
pub const SCREEN_HEIGHT: u16 = 320;
pub const ICON_WIDTH: u16 = 64;
