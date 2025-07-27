//! shared VM implementation

mod config;

/// atom/primitive types
#[derive(Copy, Clone)]
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

impl Default for Primitive {
    fn default() -> Self {
        Primitive { Nil }
    }
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

/// generic stack
/// `T` elements type
/// `S:usize` fixed size
pub struct Stack<T, const S: usize> {
    data: [T; S],
    pointer: usize,
}

impl<T, const S: usize> Stack<T, S> {
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self {
            data: [T::default(); S],
            pointer: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.data[self.pointer] = item;
        assert!(self.pointer < S);
        self.pointer += 1;
    }
    // pub fn pop(&mut self, ) -> T { assert (pointer>0) ; stack[--pointer] }
    // pub fn depth(& self, ) -> usize { pointer }
    // pub fn clear(&mut self, ) { pointer=0; }
    // pub fn empty(&self) -> bool { pointer>0; }
}

pub struct CodePoint<'a> {
    seq: &'a Seq,
    ip: usize,
}

impl CodePoint {
    pub fn new(seq: &Seq) -> Self {
        Self {
            seq: &vec![],
            ip: 0,
        }
    }
}

impl Default for CodePoint {
    fn default() -> Self {
        Self {
            seq: Default::default(),
            ip: Default::default(),
        }
    }
}

/// Virtual Machine Context:
/// - stacks
pub struct VM {
    /// data stack
    data: Stack<Primitive, { config::DSZ }>,
    /// return stack
    ret: Stack<usize, { config::RSZ }>,
    /// execution pointer
    cp: CodePoint,
}

impl VM {
    pub fn new() -> Self {
        Self {
            data: Stack::new(),
            ret: Stack::new(),
            cp: CodePoint::new(),
        }
    }

    pub fn run(&mut self, seq: &Seq) {
        let seq_len = cp.seq.len();
        while cp.ip < seq_len {
            match self.cp.seq[self.cp.ip] {
                Bytecode::Primitive(p) => self.data.push(*p),
                Bytecode::Cmd(c) => c(),
            }
            self.cp.ip += 1;
        }
    }
}
