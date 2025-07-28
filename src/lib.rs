//! shared VM implementation

#![allow(dead_code)]
#![allow(unused_imports)]

mod config;
use config::*;

/// main memory size, bytes
pub const Msz: usize = 0x10000;
/// return stack size
pub const Rsz: usize = 0x100;
/// data stack size
pub const Dsz: usize = 0x10;

/// atom/primitive types
#[derive(Copy, Clone)]
pub enum Primitive {
    /// signed integer
    Int(i32),
    /// floating point
    Float(f32),
    /// text char
    Char(char),
    /// boolean
    Bool(bool),
    /// empty element
    Nil,
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive::Nil
    }
}

/// Virtual Machine command
pub type Cmd = fn();

/// `( -- )` do nothing
fn nop() {
    trace("nop");
}

/// `( -- )` stop system
fn halt() {
    trace("halt");
    std::process::exit(0);
}

/// low-level bytecode
pub enum ByteCode {
    /// Primitive can be used as is
    Primitive(Primitive),
    /// functions can be used as VM command
    Cmd(Cmd),
}

/// executable sequence
pub type Seq = Vec<ByteCode>;

/// generic fixed size stack
/// `T` elements type
/// `S:usize` fixed size
pub struct Stack<T, const S: usize> {
    data: [T; S],
    pointer: usize,
}

// impl<T, const S: usize> Stack<T, S> {
//     pub fn new() -> Self
//     where
//         T: Default + Copy,
//     {
//         Self {
//             data: [T::default(); S],
//             pointer: 0,
//         }
//     }

//     pub fn push(&mut self, item: T) {
//         self.data[self.pointer] = item;
//         assert!(self.pointer < S);
//         self.pointer += 1;
//     }
//     // pub fn pop(&mut self, ) -> T { assert (pointer>0) ; stack[--pointer] }
//     // pub fn depth(& self, ) -> usize { pointer }
//     // pub fn clear(&mut self, ) { pointer=0; }
//     // pub fn empty(&self) -> bool { pointer>0; }
// }

/// Virtual Machine execution context
pub struct Context {
    /// data stack
    data: Stack<Primitive, { VM_DSZ }>,
    /// return stack
    ret: Stack<usize, { VM_RSZ }>,
    /// active sequence
    seq: Seq,
    /// execution pointer
    ip: usize,
}
