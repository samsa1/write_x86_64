use super::reg::{Label, Operand, Reg, RegB, RegQ};
use std::io::prelude::*;

/// Trait representing instructions
pub trait Instr {
    /// Write instruction in file
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;
}

/// Instruction using 2 operands
pub enum OpOpInstrName {
    /// Move operation
    Move,
    /// Add operation
    Add,
    /// Substration
    Sub,
    /// Bitwise And
    And,
    /// Bitwise Or
    Or,
    /// Bitwise Xor
    Xor,
    /// Left shift
    Shl,
    /// Logical right shift
    Shr,
    /// Arithmetic right shift
    Sar,
    /// Compare (set flags based on Sub instr)
    Cmp,
    /// Test  (set flags based on And instr)
    Test,
    /// Compute address and stores the address instead of value pointed
    Lea,
    /// Signed multiplication
    IMul,
}

impl OpOpInstrName {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Move => "mov",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Shl => "shl",
            Self::Shr => "shr",
            Self::Sar => "sar",
            Self::Cmp => "cmp",
            Self::Test => "test",
            Self::Lea => "lea",
            Self::IMul => "imul",
        }
    }
}

/// Instruction using 2 operands
pub struct InstrOpOp<T: Reg> {
    instr: OpOpInstrName,
    reg1: Operand<T>,
    reg2: Operand<T>,
}

impl<T: Reg> InstrOpOp<T> {
    #[doc(hidden)]
    pub fn new(instr: OpOpInstrName, reg1: Operand<T>, reg2: Operand<T>) -> Self {
        Self { instr, reg1, reg2 }
    }
}

impl<T: Reg> Instr for InstrOpOp<T> {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"\t")?;
        file.write_all(self.instr.to_str().as_bytes())?;
        file.write_all(&[T::SIZE.to_char() as u8])?;
        file.write_all(b" ")?;
        self.reg1.write_in(file)?;
        file.write_all(b", ")?;
        self.reg2.write_in(file)?;
        file.write_all(b"\n")
    }
}

/// Instruction with only one argument
pub enum OpInstrName {
    /// Increment value
    Inc,
    /// Decrement value
    Dec,
    /// Arithmetic negation
    Neg,
    /// Bitwise negation
    Not,
    /// Push on stack
    Push,
    /// Pop from stack
    Pop,
    /// Unsigned division (beware of edx/rdc register)
    UnsignedDiv,
    /// Signed division (beware of edx/rdc register)
    SignedDiv,
}

impl OpInstrName {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Inc => "inc",
            Self::Dec => "dec",
            Self::Neg => "neg",
            Self::Not => "not",
            Self::Push => "push",
            Self::Pop => "pop",
            Self::UnsignedDiv => "div",
            Self::SignedDiv => "idiv",
        }
    }
}

/// Instruction with only one argument
pub struct InstrOp<T: Reg> {
    instr: OpInstrName,
    reg: Operand<T>,
}

impl<T: Reg> InstrOp<T> {
    #[doc(hidden)]
    pub fn new(instr: OpInstrName, reg: Operand<T>) -> Self {
        Self { instr, reg }
    }
}

impl<T: Reg> Instr for InstrOp<T> {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"\t")?;
        file.write_all(self.instr.to_str().as_bytes())?;
        file.write_all(&[T::SIZE.to_char() as u8])?;
        file.write_all(b" ")?;
        self.reg.write_in(file)?;
        file.write_all(b"\n")
    }
}

/// Instruction with no argument
pub enum InstrNoArg {
    /// Equivalent to popq rip
    Ret,
    /// Complex return from call
    Leave,
    /// Syscall
    Syscall,
    /// Hlt instruction
    Hlt,
    /// Sign extend %eax into %edx::%eax
    Cltd,
    /// Sign extend %rax into %rdx::%rax
    Cqto,
    /// Does nothing
    Nop,
}

impl Instr for InstrNoArg {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"\t")?;
        match self {
            Self::Ret => file.write_all(b"ret\n"),
            Self::Leave => file.write_all(b"leave\n"),
            Self::Syscall => file.write_all(b"syscall\n"),
            Self::Hlt => file.write_all(b"hlt\n"),
            Self::Cltd => file.write_all(b"cltd\n"),
            Self::Cqto => file.write_all(b"cqto\n"),
            Self::Nop => file.write_all(b"nop\n"),
        }
    }
}

/// Various conditionals
/// 
/// Informations given as FLAGS = meaning after cmp
pub enum Cond {
    /// Same as Z
    E,
    /// ZF = equal to 0
    Z,
    /// Same as NZ
    NE,
    /// not(ZF) = equal to 0
    NZ,
    /// SF = negative
    S,
    /// not(SF) = non-negative
    NS,
    /// not(SF xor OF) and not(ZF) = greater
    G,
    /// not(SF xor OF) = greater or equal
    GE,
    /// SF xor OF = lower
    L,
    /// (SF xor OF) or ZF = lower or equal
    LE,
    /// not(CF) and not(ZF) = above (unsigned greater)
    A,
    /// not(CF) = above or equal (unsigned greater or equal)
    AE,
    /// CF = below (unsigned lower)
    B,
    /// CF or ZF = below or equal (unsigned lower or equal)
    BE,
}

impl Cond {
    fn to_str(&self) -> &'static str {
        match self {
            Self::E => "e",
            Self::Z => "z",
            Self::NE => "ne",
            Self::NZ => "nz",
            Self::S => "s",
            Self::NS => "ns",
            Self::G => "g",
            Self::GE => "ge",
            Self::L => "l",
            Self::LE => "le",
            Self::A => "a",
            Self::AE => "ae",
            Self::B => "b",
            Self::BE => "be",
        }
    }
}

/// Conditionnal move instruction
pub struct CondMove<T: Reg> {
    cond: Cond,
    reg1: Operand<T>,
    reg2: Operand<T>,
}

impl<T: Reg> CondMove<T> {
    #[doc(hidden)]
    pub fn new(cond: Cond, reg1: Operand<T>, reg2: Operand<T>) -> Self {
        Self { cond, reg1, reg2 }
    }
}

impl<T: Reg> Instr for CondMove<T> {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"\tcmov")?;
        file.write_all(self.cond.to_str().as_bytes())?;
        file.write_all(b" ")?;
        self.reg1.write_in(file)?;
        file.write_all(b", ")?;
        self.reg2.write_in(file)?;
        file.write_all(b"\n")
    }
}

/// Various instructions to move around in the code
pub enum Goto {
    /// Call label
    Call(Label),
    /// Call address
    CallStar(Operand<RegQ>),
    /// Conditionnal jump
    CondJump(Cond, Label),
    /// Jump to label
    Jump(Label),
    /// Jump to address
    JumpStar(Operand<RegQ>),
    /// Set operand to 0 or 1 based on the condition
    Set(Cond, Operand<RegB>),
}

impl Instr for Goto {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"\t")?;
        match self {
            Self::Call(label) => {
                file.write_all(b"call ")?;
                label.write_in(file)?
            }
            Self::CallStar(operand) => {
                file.write_all(b"call *")?;
                operand.write_in(file)?
            }
            Self::CondJump(cond, label) => {
                file.write_all(b"j")?;
                file.write_all(cond.to_str().as_bytes())?;
                file.write_all(b" ")?;
                label.write_in(file)?
            }
            Self::Jump(label) => {
                file.write_all(b"jmp ")?;
                label.write_in(file)?
            }
            Self::JumpStar(operand) => {
                file.write_all(b"jmp *")?;
                operand.write_in(file)?
            }
            Self::Set(cond, op) => {
                file.write_all(b"set")?;
                file.write_all(cond.to_str().as_bytes())?;
                file.write_all(b" ")?;
                op.write_in(file)?
            }
        }
        file.write_all(b"\n")
    }
}
