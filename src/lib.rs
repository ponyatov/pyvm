//! shared VM implementation

use crate::config::*;

/// atom/primitive types
pub enum Primitive {
    /// signed integer
    Int(i32),
    /// floating point
    Float(f32),
    /// boolean
    Bool(bool),
    /// text char
    Char(char),
    /// unit element
    Nil,
}

/// Virtual Machine command
pub type Cmd = fn();

pub fn nop() {}
pub fn halt() {
    std::process::exit(0);
}

/// low-level bytecode
pub enum Bytecode {
    /// any Primitive can be used as is
    Primitive(Primitive),
    /// any function can be used as VM command
    Cmd(Cmd),
}

/// executable sequence
pub type Seq = Vec<Bytecode>;

/// data stack
pub static D: [Primitive; DSZ] = [Primitive::Nil; DSZ];
