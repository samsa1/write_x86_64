use std::io::Write;

use crate::reg::{Label, Operand};
use crate::traits::{Reg, Writable};

/// Various conditionals
///
/// Informations given as FLAGS = meaning after cmp
#[derive(Debug)]
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

/// Various instructions names
#[derive(Debug)]
pub enum InstrName {
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

    /// Sign extend move
    Movs,
    /// Fill with zeros move
    Movz,
    /// Logical shift left
    ShlC,
    /// Logical shift right
    ShrC,

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
    /// Conditionnal move
    Cmov(Cond),
    /// Call label
    Call(Label),
    /// Call address
    CallStar,
    /// Conditionnal jump
    CondJump(Cond, Label),
    /// Jump to label
    Jump(Label),
    /// Jump to address
    JumpStar,
    /// Set operand to 0 or 1 based on the condition
    Set(Cond),
}

impl Writable for InstrName {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            InstrName::Move => file.write_all(b"mov"),
            InstrName::Add => file.write_all(b"add"),
            InstrName::Sub => file.write_all(b"sub"),
            InstrName::And => file.write_all(b"and"),
            InstrName::Or => file.write_all(b"or"),
            InstrName::Xor => file.write_all(b"xor"),
            InstrName::Shl => file.write_all(b"shl"),
            InstrName::Shr => file.write_all(b"shr"),
            InstrName::Sar => file.write_all(b"sar"),
            InstrName::Cmp => file.write_all(b"cmp"),
            InstrName::Test => file.write_all(b"test"),
            InstrName::Lea => file.write_all(b"lea"),
            InstrName::IMul => file.write_all(b"imul"),
            InstrName::Movs => file.write_all(b"movs"),
            InstrName::Movz => file.write_all(b"movz"),
            InstrName::ShlC => file.write_all(b"shl"),
            InstrName::ShrC => file.write_all(b"shr"),
            InstrName::Inc => file.write_all(b"inc"),
            InstrName::Dec => file.write_all(b"dec"),
            InstrName::Neg => file.write_all(b"neg"),
            InstrName::Not => file.write_all(b"not"),
            InstrName::Push => file.write_all(b"push"),
            InstrName::Pop => file.write_all(b"pop"),
            InstrName::UnsignedDiv => file.write_all(b"div"),
            InstrName::SignedDiv => file.write_all(b"idiv"),
            InstrName::Ret => file.write_all(b"ret"),
            InstrName::Leave => file.write_all(b"leave"),
            InstrName::Syscall => file.write_all(b"syscall"),
            InstrName::Hlt => file.write_all(b"hlt"),
            InstrName::Cltd => file.write_all(b"cltd"),
            InstrName::Cqto => file.write_all(b"cqto"),
            InstrName::Nop => file.write_all(b"nop"),
            InstrName::Cmov(cond) => {
                file.write_all(b"cmov")?;
                file.write_all(cond.to_str().as_bytes())
            }
            InstrName::Call(label) => {
                file.write_all(b"call ")?;
                label.write_in(file)
            }
            InstrName::CallStar => file.write_all(b"call *"),
            InstrName::CondJump(cond, label) => {
                file.write_all(b"j")?;
                file.write_all(cond.to_str().as_bytes())?;
                file.write_all(b" ")?;
                label.write_in(file)
            }
            InstrName::Jump(label) => {
                file.write_all(b"jmp ")?;
                label.write_in(file)
            }
            InstrName::JumpStar => file.write_all(b"jmp *"),
            InstrName::Set(cond) => {
                file.write_all(b"set")?;
                file.write_all(cond.to_str().as_bytes())
            }
        }
    }
}

impl InstrName {
    fn nb_args(&self) -> usize {
        match self {
            InstrName::Move
            | InstrName::Add
            | InstrName::Sub
            | InstrName::And
            | InstrName::Or
            | InstrName::Xor
            | InstrName::Shl
            | InstrName::Shr
            | InstrName::Sar
            | InstrName::Cmp
            | InstrName::Test
            | InstrName::Lea
            | InstrName::IMul => 2,
            InstrName::Movs | InstrName::Movz | InstrName::ShlC | InstrName::ShrC => 2,
            InstrName::Inc
            | InstrName::Dec
            | InstrName::Neg
            | InstrName::Not
            | InstrName::Push
            | InstrName::Pop
            | InstrName::UnsignedDiv
            | InstrName::SignedDiv => 1,
            InstrName::Ret
            | InstrName::Leave
            | InstrName::Syscall
            | InstrName::Hlt
            | InstrName::Cltd
            | InstrName::Cqto
            | InstrName::Nop => 0,
            InstrName::Cmov(_) => 2,
            InstrName::Call(_) => 0,
            InstrName::CallStar => 1,
            InstrName::CondJump(_, _) => 0,
            InstrName::Jump(_) => 0,
            InstrName::JumpStar => 1,
            InstrName::Set(_) => 1,
        }
    }

    fn print_size_1(&self) -> bool {
        match self {
            InstrName::Move
            | InstrName::Add
            | InstrName::Sub
            | InstrName::And
            | InstrName::Or
            | InstrName::Xor
            | InstrName::Shl
            | InstrName::Shr
            | InstrName::Sar
            | InstrName::Cmp
            | InstrName::Test
            | InstrName::Lea
            | InstrName::IMul => false,
            InstrName::Movs | InstrName::Movz => false,
            InstrName::ShlC | InstrName::ShrC => true,
            InstrName::Inc
            | InstrName::Dec
            | InstrName::Neg
            | InstrName::Not
            | InstrName::Push
            | InstrName::Pop
            | InstrName::UnsignedDiv
            | InstrName::SignedDiv => true,
            InstrName::Ret
            | InstrName::Leave
            | InstrName::Syscall
            | InstrName::Hlt
            | InstrName::Cltd
            | InstrName::Cqto
            | InstrName::Nop => false,
            InstrName::Cmov(_) => false,
            InstrName::Call(_) => false,
            InstrName::CallStar => false,
            InstrName::CondJump(_, _) => false,
            InstrName::Jump(_) => false,
            InstrName::JumpStar => false,
            InstrName::Set(_) => false,
        }
    }

    fn print_size_2(&self) -> bool {
        match self {
            InstrName::Move
            | InstrName::Add
            | InstrName::Sub
            | InstrName::And
            | InstrName::Or
            | InstrName::Xor
            | InstrName::Shl
            | InstrName::Shr
            | InstrName::Sar
            | InstrName::Cmp
            | InstrName::Test
            | InstrName::Lea
            | InstrName::IMul => true,
            InstrName::Movs | InstrName::Movz => true,
            InstrName::ShlC | InstrName::ShrC => true,
            InstrName::Inc
            | InstrName::Dec
            | InstrName::Neg
            | InstrName::Not
            | InstrName::Push
            | InstrName::Pop
            | InstrName::UnsignedDiv
            | InstrName::SignedDiv => false,
            InstrName::Ret
            | InstrName::Leave
            | InstrName::Syscall
            | InstrName::Hlt
            | InstrName::Cltd
            | InstrName::Cqto
            | InstrName::Nop => false,
            InstrName::Cmov(_) => false,
            InstrName::Call(_) => false,
            InstrName::CallStar => false,
            InstrName::CondJump(_, _) => false,
            InstrName::Jump(_) => false,
            InstrName::JumpStar => false,
            InstrName::Set(_) => false,
        }
    }

    fn add_space(&self) -> bool {
        match self {
            InstrName::CallStar | InstrName::JumpStar => false,
            _ => true,
        }
    }
}

/// Structure storing the instruction name and a most 2 operands.
/// To type with less than 2 operands use the type RegInv which can never be used for real operands
pub struct Instruction<S1: Reg = crate::reg::RegInv, S2: Reg = crate::reg::RegInv> {
    /// Instruction name
    pub instr: InstrName,
    /// First operand if exists
    pub reg1: Option<Operand<S1>>,
    /// Second operand if exists
    pub reg2: Option<Operand<S2>>,
}

/// Trait to wrap around all instructions types
pub trait InstrTrait {
    /// Write the instruction in a file
    fn write_instr(&self, file: &mut std::fs::File) -> std::io::Result<()>;
}

impl<S1: Reg, S2: Reg> InstrTrait for Instruction<S1, S2> {
    fn write_instr(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.instr.write_in(file)?;
        if self.instr.print_size_1() {
            file.write_all(&[S1::SIZE.to_char() as u8])?;
        }
        if self.instr.print_size_2() {
            file.write_all(&[S2::SIZE.to_char() as u8])?;
        }
        if self.instr.nb_args() >= 1 {
            if self.instr.add_space() {
                file.write_all(b" ")?
            };
            self.reg1.as_ref().unwrap().write_in(file)?;
        } else {
            if self.reg1.is_some() {
                panic!(
                    "Instruction {:?} expects 0 arguments but received a least 1",
                    self.instr
                )
            }
        }
        if self.instr.nb_args() >= 2 {
            file.write_all(b", ")?;
            self.reg2.as_ref().unwrap().write_in(file)?;
        } else {
            if self.reg2.is_some() {
                panic!(
                    "Instruction {:?} expects at most 1 arguments but received a second argument",
                    self.instr
                )
            }
        }
        std::io::Result::Ok(())
    }
}

/// Type representing an instruction
pub type Instr = Box<dyn InstrTrait>;

impl Writable for Instr {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.write_instr(file)
    }
}
