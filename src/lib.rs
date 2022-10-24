//! Module to help people generate x86_64 code in Rust
//!
//! Inspired from <https://www.lri.fr/~filliatr/ens/compil/lib/x86_64.ml.html>
//!
//! This crate wants to be an equivalent of the above code
//!
//! For writting files see
//!
//! [`data::Data`], [`file::File`] and [Text] data structures
//!
//! Registers %rax -> %r15 are all accessible for 8, 16, 32 and 64 bits.
//! Write name in capital letters to access them
//!
//! Operands can be obtained with [`lab`], [`ilab`], [`addr`], [`reg!`] and [`immb`] to [`immq`].
//!
//! All instruction are available for various sizes.
//!
//! Transfert instruction : [`movq`]
//!
//! Arithmetic : [`leaq`], [`incq`], [`decq`], [`negq`], [`addq`], [`subq`], [`imulq`], [`cqto`], [`idivq`], [`divq`]
//!
//! Logic : [`notq`], [`andq`], [`orq`], [`xorq`]
//!
//! Shifts : [`shlq`], [`shrq`], [`sarq`]
//!
//! Jumps : [`call`], [`call_star`], [`leave`], [`ret`], [`jmp`], [`jmp_star`], [`jcc`]
//!
//! Conditions : [`cmpb`], [`testq`], [`set`], [`cmovq`]
//!
//! Stack : [`pushq`], [`popq`]
//!
//! Various others : [`label`], [`comment`]

// Author :
// 2022 Samuel VIVIEN

#![warn(missing_docs)]

/// Define data segment
pub mod data;

/// Define structure for whole file
pub mod file;

/// Defines instructions
pub mod instr;

/// Defines registers and operands
pub mod reg;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

use std::io::prelude::*;
use std::ops::Add;

// Code

/// nop instruction (does nothing)
pub fn nop() -> Text {
    Text::Instr(Box::new(instr::InstrNoArg::Nop))
}

/// Data structure representing assembly
///
/// It is recommended to not build this type yourself but instead use the functions provided
pub enum Text {
    /// Concatenation of assembly code
    Concat(Vec<Text>),
    /// Instruction
    Instr(Box<dyn instr::Instr>),
    /// Label
    Label(reg::Label),

    /// Comment
    Comment(String),
}

impl Add for Text {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Concat(vec![self, other])
    }
}

impl Text {
    fn write_in(self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::Concat(vec) => {
                for asm in vec {
                    asm.write_in(file)?
                }
                Ok(())
            }
            Self::Label(label) => {
                label.write_in(file)?;
                file.write_all(b":\n")
            }
            Self::Instr(instr) => instr.write_in(file),
            Self::Comment(comment) => {
                file.write_all(b"\\\\")?;
                file.write_all(comment.as_bytes())
            }
        }
    }
}

//// Registers

def_regq!(RAX, Rax);
def_regq!(RBX, Rbx);
def_regq!(RCX, Rcx);
def_regq!(RDX, Rdx);
def_regq!(RSI, Rsi);
def_regq!(RDI, Rdi);
def_regq!(RBP, Rbp);
def_regq!(RSP, Rsp);
def_regq!(R8, R8);
def_regq!(R9, R9);
def_regq!(R10, R10);
def_regq!(R11, R11);
def_regq!(R12, R12);
def_regq!(R13, R13);
def_regq!(R14, R14);
def_regq!(R15, R15);

def_regl!(EAX, Eax);
def_regl!(EBX, Ebx);
def_regl!(ECX, Ecx);
def_regl!(EDX, Edx);
def_regl!(ESI, Esi);
def_regl!(EDI, Edi);
def_regl!(EBP, Ebp);
def_regl!(ESP, Esp);
def_regl!(R8D, R8d);
def_regl!(R9D, R9d);
def_regl!(R10D, R10d);
def_regl!(R11D, R11d);
def_regl!(R12D, R12d);
def_regl!(R13D, R13d);
def_regl!(R14D, R14d);
def_regl!(R15D, R15d);

def_regw!(AX, Ax);
def_regw!(BX, Bx);
def_regw!(CX, Cx);
def_regw!(DX, Dx);
def_regw!(SI, Si);
def_regw!(DI, Di);
def_regw!(BP, Bp);
def_regw!(SP, Sp);
def_regw!(R8W, R8w);
def_regw!(R9W, R9w);
def_regw!(R10W, R10w);
def_regw!(R11W, R11w);
def_regw!(R12W, R12w);
def_regw!(R13W, R13w);
def_regw!(R14W, R14w);
def_regw!(R15W, R15w);

def_regb!(AL, Al);
def_regb!(BL, Bl);
def_regb!(CL, Cl);
def_regb!(DL, Dl);
def_regb!(AH, Ah);
def_regb!(BH, Bh);
def_regb!(CH, Ch);
def_regb!(DH, Dh);
def_regb!(SIL, Sil);
def_regb!(DIL, Dil);
def_regb!(BPL, Bpl);
def_regb!(SPL, Spl);
def_regb!(R8B, R8b);
def_regb!(R9B, R9b);
def_regb!(R10B, R10b);
def_regb!(R11B, R11b);
def_regb!(R12B, R12b);
def_regb!(R13B, R13b);
def_regb!(R14B, R14b);
def_regb!(R15B, R15b);

/// Operands

/// Immediate operand for 64-bits instructions
pub fn immq(imm: i64) -> reg::Operand<reg::RegQ> {
    reg::Operand::Imm(imm)
}

/// Immediate operand for 32-bits instructions
pub fn imml(imm: i32) -> reg::Operand<reg::RegL> {
    reg::Operand::Imm(imm as i64)
}

/// Immediate operand for 16-bits instructions
pub fn immw(imm: i16) -> reg::Operand<reg::RegW> {
    reg::Operand::Imm(imm as i64)
}

/// Immediate operand for 8-bits instructions
pub fn immb(imm: i8) -> reg::Operand<reg::RegB> {
    reg::Operand::Imm(imm as i64)
}

/// Macro to convert an element of type R with the trait Reg
/// to an element of type Operand<R>
#[macro_export]
macro_rules! reg {
    ($reg:expr) => {
        $crate::reg::Operand::Reg($reg)
    };
}

/// Create an Operand<R> (for any type R) to access memory
///
/// addr!(rsp) => (%rsp)
///
/// addr!(offset, rbp) => offset(%rbp)
///
/// addr!(offset, rbp, rax) => offset(%rbp, %rax, 1)
///
/// addr!(offset, rbp, rax, scale) => offset(%rbp, %rax, scale)
#[macro_export]
macro_rules! addr {
    ($reg:expr) => {
        $crate::reg::Operand::Addr(0, $reg, None, 0)
    };
    ($offset:expr, $reg:expr) => {
        $crate::reg::Operand::Addr($offset, $reg, None, 0)
    };
    ($offset:expr, $reg:expr, $reg2:expr) => {
        $crate::reg::Operand::Addr($offset, $reg, Some($reg2), 1)
    };
    ($offset:expr, $reg:expr, $reg2:expr, $scale:expr) => {
        $crate::reg::Operand::Addr($offset, $reg, Some($reg2), scale)
    };
}

#[cfg(target_os = "macos")]
#[macro_export]
/// lab operator from <https://www.lri.fr/~filliatr/ens/compil/lib/x86_64.ml.html>
macro_rules! lab {
    ($label:expr) => {
        $crate::reg::Operand::LabRelAddr($label)
    };
}

#[cfg(target_os = "linux")]
#[macro_export]
/// lab operator from <https://www.lri.fr/~filliatr/ens/compil/lib/x86_64.ml.html>
macro_rules! lab {
    ($label:expr) => {
        $crate::reg::Operand::LabAbsAddr($label)
    };
}

#[macro_export]
/// ilab operator from <https://www.lri.fr/~filliatr/ens/compil/lib/x86_64.ml.html>
macro_rules! ilab {
    ($label:expr) => {
        $crate::reg::Operand::LabVal($label)
    };
}

//// Instructions

// Data transfer

build_instr_op_op!(Move, movb, movw, movl, movq);

/// Sign extend for 1-byte to 2-bytes
pub fn movsbw(reg1: reg::Operand<reg::RegB>, reg2: reg::RegW) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Sign extend for 1-byte to 4-bytes
pub fn movsbl(reg1: reg::Operand<reg::RegB>, reg2: reg::RegL) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Sign extend for 1-byte to 8-bytes
pub fn movsbq(reg1: reg::Operand<reg::RegB>, reg2: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Sign extend for 2-byte to 4-bytes
pub fn movswl(reg1: reg::Operand<reg::RegW>, reg2: reg::RegL) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Sign extend for 2-byte to 8-bytes
pub fn movswq(reg1: reg::Operand<reg::RegW>, reg2: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Sign extend for 4-byte to 8-bytes
pub fn movslq(reg1: reg::Operand<reg::RegL>, reg2: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movs,
        reg1,
        reg!(reg2),
    )))
}

/// Extension with zeros for 1-byte to 2-bytes
pub fn movzbw(reg1: reg::Operand<reg::RegB>, reg2: reg::RegW) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movz,
        reg1,
        reg!(reg2),
    )))
}

/// Extension with zeros for 1-byte to 4-bytes
pub fn movzbl(reg1: reg::Operand<reg::RegB>, reg2: reg::RegL) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movz,
        reg1,
        reg!(reg2),
    )))
}

/// Extension with zeros for 1-byte to 8-bytes
pub fn movzbq(reg1: reg::Operand<reg::RegB>, reg2: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movz,
        reg1,
        reg!(reg2),
    )))
}

/// Extension with zeros for 2-byte to 4-bytes
pub fn movzwl(reg1: reg::Operand<reg::RegW>, reg2: reg::RegL) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movz,
        reg1,
        reg!(reg2),
    )))
}

/// Extension with zeros for 2-byte to 8-bytes
pub fn movzwq(reg1: reg::Operand<reg::RegW>, reg2: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Movz,
        reg1,
        reg!(reg2),
    )))
}

// Move between different sizes not implemented

// movabsq not implemented

//// Arithmetic

build_instr_op_reg!(Lea, leab, leaw, leal, leaq);

build_instr_op!(Inc, incb, incw, incl, incq);

build_instr_op!(Dec, decb, decw, decl, decq);

build_instr_op!(Neg, negb, negw, negl, negq);

build_instr_op_op!(Add, addb, addw, addl, addq);

build_instr_op_op!(Sub, subb, subw, subl, subq);

build_instr_op_op!(IMul, imulw, imull, imulq);

/// sign extend EAX into EDX::EAX
pub fn cltd() -> Text {
    Text::Instr(Box::new(instr::InstrNoArg::Cltd))
}

/// sign extend RAX into RDX::RAX
pub fn cqto() -> Text {
    Text::Instr(Box::new(instr::InstrNoArg::Cqto))
}

build_instr_op!(SignedDiv, idivl, idivq);

build_instr_op!(UnsignedDiv, divl, divq);

///// Logic operations
// Those operations are bitwise operations

build_instr_op!(Not, notb, notw, notl, notq);

build_instr_op_op!(And, andb, andw, andl, andq);

build_instr_op_op!(Or, orb, orw, orl, orq);

build_instr_op_op!(Xor, xorb, xorw, xorl, xorq);

//// Shifts

build_instr_op_op!(Shl, shlb, shlw, shll, shlq);
build_instr_op_op!(Shr, shrb, shrw, shrl, shrq);
build_instr_op_op!(Sar, sarb, sarw, sarl, sarq);

/// logical shift of register by value in CL
pub fn shlb_reg(reg1: reg::Operand<reg::RegB>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shl,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shlw_reg(reg1: reg::Operand<reg::RegW>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shl,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shll_reg(reg1: reg::Operand<reg::RegL>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shl,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shlq_reg(reg1: reg::Operand<reg::RegQ>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shl,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shrb_reg(reg1: reg::Operand<reg::RegB>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shr,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shrw_reg(reg1: reg::Operand<reg::RegW>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shr,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shrl_reg(reg1: reg::Operand<reg::RegL>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shr,
        reg1,
        reg!(CL),
    )))
}

/// logical shift of register by value in CL
pub fn shrq_reg(reg1: reg::Operand<reg::RegQ>) -> Text {
    Text::Instr(Box::new(instr::InstrOpOpDif::new(
        instr::OpOpDifInstrName::Shr,
        reg1,
        reg!(CL),
    )))
}

//// Jumps

// Function calls and return

/// Call label
pub fn call(label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::Call(label)))
}

/// Call address
pub fn call_star(op: reg::Operand<reg::RegQ>) -> Text {
    Text::Instr(Box::new(instr::Goto::CallStar(op)))
}

/// Leave instruction
pub fn leave() -> Text {
    Text::Instr(Box::new(instr::InstrNoArg::Leave))
}

/// Equivalent to popq %rip
pub fn ret() -> Text {
    Text::Instr(Box::new(instr::InstrNoArg::Ret))
}

/// Jump to label
pub fn jmp(label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::Jump(label)))
}

/// Jump to address
pub fn jmp_star(op: reg::Operand<reg::RegQ>) -> Text {
    Text::Instr(Box::new(instr::Goto::JumpStar(op)))
}

////// Conditional jumps

/// Conditional jump
pub fn jcc(cond: instr::Cond, label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::CondJump(cond, label)))
}

/// Conditional jump if zero
pub fn jz(label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::CondJump(instr::Cond::Z, label)))
}

/// Conditional jump if not zero
pub fn jnz(label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::CondJump(instr::Cond::NZ, label)))
}

/// Conditional jump if above equal
pub fn jae(label: reg::Label) -> Text {
    Text::Instr(Box::new(instr::Goto::CondJump(instr::Cond::AE, label)))
}

//// Conditions

build_instr_op_op!(Cmp, cmpb, cmpw, cmpl, cmpq);

build_instr_op_op!(Test, testb, testw, testl, testq);

/// Conditionnal set
pub fn set(cond: instr::Cond, reg: reg::Operand<reg::RegB>) -> Text {
    Text::Instr(Box::new(instr::Goto::Set(cond, reg)))
}

//// Stack handling

/// Push 8-bytes on stack
pub fn pushq(op: reg::Operand<reg::RegQ>) -> Text {
    Text::Instr(Box::new(instr::InstrOp::new(instr::OpInstrName::Push, op)))
}

/// Pop 8-bytes from stack
pub fn popq(op: reg::RegQ) -> Text {
    Text::Instr(Box::new(instr::InstrOp::new(
        instr::OpInstrName::Pop,
        reg::Operand::Reg(op),
    )))
}

//// Various others

/// Place a label
pub fn label(l: reg::Label) -> Text {
    Text::Label(l)
}

/// Add comment to Assembly (should not contain de line break!)
pub fn comment(s: String) -> Text {
    Text::Comment(s)
}

#[cfg(target_os = "linux")]
/// Move address of label in register (implementation is OS dependant)
///
/// Usefull to get address to string before calling printf
///
/// Not sure it works on linux, needs to test!
pub fn deplq(l: reg::Label, reg: reg::RegQ) -> Text {
    movq(reg::Operand::LabAbsAddr(l), reg::Operand::Reg(reg))
}

#[cfg(target_os = "macos")]
/// Move address of label in register (implementation is OS dependant)
///
/// Usefull to get address to string before calling printf
///
/// Not sure it works on linux, needs to test!
pub fn deplq(l: reg::Label, reg: reg::RegQ) -> Text {
    leaq(reg::Operand::LabRelAddr(l), reg)
}

// cmovb is not valid

/// Conditional move of 2-bytes operands
pub fn cmovw(
    cond: instr::Cond,
    reg1: reg::Operand<reg::RegW>,
    reg2: reg::Operand<reg::RegW>,
) -> Text {
    Text::Instr(Box::new(instr::CondMove::new(cond, reg1, reg2)))
}

/// Conditional move of 4-bytes operands
pub fn cmovl(
    cond: instr::Cond,
    reg1: reg::Operand<reg::RegL>,
    reg2: reg::Operand<reg::RegL>,
) -> Text {
    Text::Instr(Box::new(instr::CondMove::new(cond, reg1, reg2)))
}

/// Conditional move of 8-bytes operands
pub fn cmovq(
    cond: instr::Cond,
    reg1: reg::Operand<reg::RegQ>,
    reg2: reg::Operand<reg::RegQ>,
) -> Text {
    Text::Instr(Box::new(instr::CondMove::new(cond, reg1, reg2)))
}

/// Convert str to label name
pub fn new_label(name: &str) -> reg::Label {
    reg::Label::from_str(name.to_string())
}
