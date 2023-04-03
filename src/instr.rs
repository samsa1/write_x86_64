use std::io::Write;

use crate::reg::{Label, Operand, RegInv, RegQ, Sizes};
use crate::traits::{Reg, Writable};

/// Various conditionals
///
/// Informations given as FLAGS = meaning after cmp
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Add with carry
    Adc,
    /// Substration
    Sub,
    /// Substration with borrow
    Sbb,
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
            InstrName::Adc => file.write_all(b"adc"),
            InstrName::Sub => file.write_all(b"sub"),
            InstrName::Sbb => file.write_all(b"sbb"),
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
            | InstrName::Adc
            | InstrName::Sub
            | InstrName::Sbb
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
            | InstrName::Adc
            | InstrName::Sub
            | InstrName::Sbb
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
            InstrName::Movs | InstrName::Movz => true,
            InstrName::ShlC | InstrName::ShrC => false,
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
            | InstrName::Adc
            | InstrName::Sub
            | InstrName::Sbb
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

struct REX {
    w: bool,
    r: bool,
    x: bool,
    b: bool,
}

impl REX {
    fn new() -> Self {
        Self {
            w: false,
            /// extends MODRM.reg
            r: false,
            /// extends SIB.index
            x: false,
            /// extends MODRM.rm or the SIB.base
            b: false,
        }
    }

    fn needed(&self) -> bool {
        self.w || self.r || self.x || self.b
    }

    fn as_byte(&self) -> u8 {
        let mut out = 0x40;
        if self.w {
            out |= 0x08;
        }
        if self.r {
            out |= 0x4;
        }
        if self.x {
            out |= 0x2;
        }
        if self.b {
            out |= 0x1;
        }
        out
    }
}

enum Imm {
    NoImm(u8),
    I8(i8, u8),
    I16(i16, u8),
    I32(i32, u8),
    I64(i64, u8),
}

enum RegOrQ<R = RegInv> {
    R(R),
    Q(RegQ),
}

impl<R: Reg> RegOrQ<R> {
    fn to_bits(&self) -> (bool, u8) {
        match self {
            Self::R(r) => r.to_bits(),
            Self::Q(r) => r.to_bits(),
        }
    }
}

struct ByteCode<R1: Reg = RegInv, R2: Reg = RegInv> {
    small_reg_flag: bool,
    prefix: Option<u8>,
    op_code: u8,
    rex: REX,
    reg: Option<R1>,
    rm: RegOrQ<R2>,
    addr: Option<(i64, Option<(RegQ, u8)>)>,
    imm: Imm,
}

impl<R2: Reg> ByteCode<RegInv, R2> {
    fn only_rm(rex: REX, rm: &Operand<R2>) -> Self {
        let mut addr = None;
        let rm = match rm {
            Operand::Addr(offset, base, None, 0) => {
                addr = Some((*offset, None));
                RegOrQ::Q(base.clone())
            }
            Operand::Addr(offset, base, Some(ind), scale) => {
                addr = Some((*offset, Some((ind.clone(), *scale))));
                RegOrQ::Q(base.clone())
            }
            Operand::Reg(rm) => RegOrQ::R(rm.clone()),
            _ => panic!("Should not happen"),
        };
        Self {
            small_reg_flag: false,
            prefix: None,
            op_code: 0,
            rex,
            reg: None,
            rm,
            addr,
            imm: Imm::NoImm(0),
        }
    }
}

impl<R1: Reg, R2: Reg> ByteCode<R1, R2> {
    fn new(rex: REX, op_code: u8, reg: R1, rm: &Operand<R2>) -> Self {
        let mut addr = None;
        let rm = match rm {
            Operand::Addr(offset, base, None, 0) => {
                addr = Some((*offset, None));
                RegOrQ::Q(base.clone())
            }
            Operand::Addr(offset, base, Some(ind), scale) => {
                addr = Some((*offset, Some((ind.clone(), *scale))));
                RegOrQ::Q(base.clone())
            }
            Operand::Reg(rm) => RegOrQ::R(rm.clone()),
            _ => panic!("Should not happen"),
        };
        Self {
            small_reg_flag: false,
            prefix: None,
            op_code,
            rex,
            reg: Some(reg),
            rm,
            addr,
            imm: Imm::NoImm(0),
        }
    }

    fn as_bytes(&mut self) -> Vec<u8> {
        let mut vec = Vec::new();
        if self.small_reg_flag {
            vec.push(0x66)
        }
        let (r, reg) = match &self.reg {
            None => match &self.imm {
                Imm::NoImm(code) => (false, *code),
                Imm::I8(_, o) | Imm::I16(_, o) | Imm::I32(_, o) | Imm::I64(_, o) => (false, *o),
            },
            Some(reg) => reg.to_bits(),
        };
        let (b, base) = self.rm.to_bits();
        self.rex.b = b;
        self.rex.r = r;
        if self.rex.needed() {
            vec.push(self.rex.as_byte());
        }
        if let Some(pref) = self.prefix {
            vec.push(pref)
        }
        vec.push(self.op_code);
        match &self.addr {
            None => {
                assert!(matches!(self.rm, RegOrQ::R(_)));
                vec.push(0b11_000_000 | (reg << 3) | base)
            }
            Some((offset, None)) => {
                assert!(matches!(self.rm, RegOrQ::Q(_)));
                if *offset == 0 && base != 4 && base != 5 {
                    vec.push(0b00_000_000 | (reg << 3) | base)
                } else if base == 4 {
                    if i8::MIN as i64 <= *offset && *offset <= i8::MAX as i64 {
                        vec.push(0b01_000_000 | (reg << 3) | base);
                        vec.push((0 << 6) | (0b100 << 3) | base);
                        vec.push(*offset as u8);
                    } else {
                        vec.push(0b10_000_000 | (reg << 3) | base);
                        vec.push((0 << 6) | (0b100 << 3) | base);
                        vec.push((*offset & 255) as u8);
                        vec.push(((*offset >> 8) & 255) as u8);
                        vec.push(((*offset >> 16) & 255) as u8);
                        vec.push(((*offset >> 24) & 255) as u8);
                    }
                } else if i8::MIN as i64 <= *offset && *offset <= i8::MAX as i64 {
                    vec.push(0b01_000_000 | (reg << 3) | base);
                    vec.push(*offset as u8);
                } else if i32::MIN as i64 <= *offset && *offset <= i32::MAX as i64 {
                    vec.push(0b10_000_000 | (reg << 3) | base);
                    vec.push((*offset & 255) as u8);
                    vec.push(((*offset >> 8) & 255) as u8);
                    vec.push(((*offset >> 16) & 255) as u8);
                    vec.push(((*offset >> 24) & 255) as u8);
                } else {
                    todo!()
                }
            }
            Some((offset, Some((ind, scale)))) => {
                let scale = match *scale {
                    1 => 0,
                    2 => 1,
                    4 => 2,
                    8 => 3,
                    _ => panic!("Invalid scale {}", scale),
                };
                assert!(ind.to_bits() != (false, 0b100));
                let (x, ind) = ind.to_bits();
                if x {
                    self.rex.x = x;
                    panic!("Not yet handled, please only use register of the first half (not r8->r15) as index");
                }
                vec.push(0b10_000_000 | (reg << 3) | base);
                vec.push((scale << 6) | (ind << 3) | base);
                vec.push((*offset & 255) as u8);
                vec.push(((*offset >> 8) & 255) as u8);
                vec.push(((*offset >> 16) & 255) as u8);
                vec.push(((*offset >> 24) & 255) as u8);
            }
        }
        match self.imm {
            Imm::NoImm(_) => (),
            Imm::I8(i, _) => vec.push(i as u8),
            Imm::I16(i, _) => {
                vec.push((i & 255) as u8);
                vec.push(((i >> 8) & 255) as u8);
            }
            Imm::I32(i, _) => {
                vec.push((i & 255) as u8);
                vec.push(((i >> 8) & 255) as u8);
                vec.push(((i >> 16) & 255) as u8);
                vec.push(((i >> 24) & 255) as u8);
            }
            Imm::I64(i, _) => {
                vec.push((i & 255) as u8);
                vec.push(((i >> 8) & 255) as u8);
                vec.push(((i >> 16) & 255) as u8);
                vec.push(((i >> 24) & 255) as u8);
                vec.push(((i >> 32) & 255) as u8);
                vec.push(((i >> 40) & 255) as u8);
                vec.push(((i >> 48) & 255) as u8);
                vec.push(((i >> 56) & 255) as u8);
            }
        }
        vec
    }
}

#[allow(dead_code)]
impl<R1: Reg, R2: Reg> Instruction<R1, R2> {
    fn to_bin(&self) -> Option<Vec<u8>> {
        match self.instr {
            InstrName::Move => {
                let op1 = self.reg1.as_ref().unwrap();
                let op2 = self.reg2.as_ref().unwrap();
                let mut rex = REX::new();
                match (R1::SIZE, R2::SIZE, op1, op2) {
                    (Sizes::Word, Sizes::Word, Operand::Reg(reg), rm)
                    | (Sizes::Long, Sizes::Long, Operand::Reg(reg), rm)
                    | (Sizes::Quad, Sizes::Quad, Operand::Reg(reg), rm) if rm.is_rm() => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::new(rex, 0x89, reg.clone(), rm);
                        op.small_reg_flag = R1::SIZE == Sizes::Word;
                        Some(op.as_bytes())
                    },
                    (Sizes::Word, Sizes::Word, rm, Operand::Reg(reg))
                    | (Sizes::Long, Sizes::Long, rm, Operand::Reg(reg))
                    | (Sizes::Quad, Sizes::Quad, rm, Operand::Reg(reg)) if rm.is_rm() => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::new(rex, 0x8b, reg.clone(), rm);
                        op.small_reg_flag = R1::SIZE == Sizes::Word;
                        Some(op.as_bytes())
                    },
                    (Sizes::Word, Sizes::Word, Operand::Imm(imm), rm) => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::only_rm(rex, rm);
                        assert!(*imm <= i16::MAX as i64 && *imm >= i16::MIN as i64);
                        op.op_code = 0xc7;
                        op.imm = Imm::I16(*imm as i16, 0);
                        op.small_reg_flag = true;
                        Some(op.as_bytes())
                    },

                    (Sizes::Long, Sizes::Long, Operand::Imm(imm), rm)
                    | (Sizes::Quad, Sizes::Quad, Operand::Imm(imm), rm) => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::only_rm(rex, rm);
                        if *imm <= i32::MAX as i64 && *imm >= i32::MIN as i64 {
                            op.op_code = 0xc7;
                            op.imm = Imm::I32(*imm as i32, 0);
                            Some(op.as_bytes())
                        } else {
                            None
                            // todo!()
                        }
                        // Some(op.as_bytes())
                    },
                    _ => None,
                }

            }
            InstrName::Add | InstrName::Or
            | InstrName::Adc | InstrName::Sbb
            | InstrName::And | InstrName::Sub
            | InstrName::Xor | InstrName::Cmp => { // https://www.felixcloutier.com/x86/add
                let op_index = match self.instr {
                    InstrName::Add => 0,
                    InstrName::Or  => 1,
                    InstrName::Adc => 2,
                    InstrName::Sbb => 3,
                    InstrName::And => 4,
                    InstrName::Sub => 5,
                    InstrName::Xor => 6,
                    InstrName::Cmp => 7,
                    _ => panic!("Should never happen"),
                };
                let _imm_ra8 = op_index * 8 + 4;
                let _imm_ra  = op_index * 8 + 5;
                let r_rm8   = op_index * 8 + 0;
                let r_rm    = op_index * 8 + 1;
                let rm_r8   = op_index * 8 + 2;
                let rm_r    = op_index * 8 + 3;
                let op1 = self.reg1.as_ref().unwrap();
                let op2 = self.reg2.as_ref().unwrap();
                let mut rex = REX::new();
                match (R1::SIZE, R2::SIZE, op1, op2) {
                    (Sizes::Byte, Sizes::Byte, rm, Operand::Reg(reg)) if rm.is_rm() => {
                        let mut op = ByteCode::new(rex, rm_r8, reg.clone(), rm);
                        Some(op.as_bytes())
                    }
                    (Sizes::Byte, Sizes::Byte, Operand::Reg(reg), rm) if rm.is_rm() => {
                        let mut op = ByteCode::new(rex, r_rm8, reg.clone(), rm);
                        Some(op.as_bytes())
                    }
                    (Sizes::Long, Sizes::Long, rm, Operand::Reg(reg))
                    | (Sizes::Quad, Sizes::Quad, rm, Operand::Reg(reg)) if rm.is_rm() => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::new(rex, rm_r, reg.clone(), rm);
                        Some(op.as_bytes())
                    }
                    (Sizes::Long, Sizes::Long, Operand::Reg(reg), rm)
                    | (Sizes::Quad, Sizes::Quad, Operand::Reg(reg), rm) if rm.is_rm() => {
                        rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::new(rex, r_rm, reg.clone(), rm);
                        Some(op.as_bytes())
                    }
                    (Sizes::Long, Sizes::Long, Operand::Imm(imm), rm)
                    | (Sizes::Quad, Sizes::Quad, Operand::Imm(imm), rm) => {
                            rex.w = R1::SIZE == Sizes::Quad;
                        let mut op = ByteCode::only_rm(rex, rm);
                        op.op_code = 0x81;
                        assert!(i32::MIN as i64 <= *imm && *imm <= i32::MAX as i64);
                        op.imm = Imm::I32(*imm as i32, op_index);
                        Some(op.as_bytes())
                    }
                    (Sizes::Quad, Sizes::Quad, Operand::LabRelAddr(_), Operand::Reg(_)) => {
                        None
                    }
                    // (Sizes::Quad, Sizes::Quad, _, _) => panic!("{:?} {:?}", self.reg1, self.reg2),
                    _ => None,
                }

            }
            InstrName::Shl =>  None,
            InstrName::Shr =>  None,
            InstrName::Sar =>  None,
            InstrName::Test => None,
            InstrName::Lea =>  None,
            InstrName::IMul => None,
            InstrName::Movs => None,
            InstrName::Movz => None,
            InstrName::ShlC => None,
            InstrName::ShrC => None,
            InstrName::Inc =>  None,
            InstrName::Dec =>  None,
            InstrName::Push => {
                let reg1 = self.reg1.as_ref().unwrap();
                let rex = REX::new();
                assert!(self.reg2.is_none());
                match (R1::SIZE, reg1) {
                    (Sizes::Byte, Operand::Imm(imm)) => {
                        assert!(i8::MIN as i64 <= *imm && *imm <= i8::MAX as i64);
                        Some(vec![0x6A, *imm as u8])
                    },
                    (Sizes::Word, Operand::Imm(imm)) => {
                        assert!(i16::MIN as i64 <= *imm && *imm <= i16::MAX as i64);
                        Some(vec![0x66, 0x68, (*imm & 255) as u8, (*imm >> 8) as u8])
                    }
                    (Sizes::Long, Operand::Imm(imm)) => {
                        assert!(i32::MIN as i64 <= *imm && *imm <= i32::MAX as i64);
                        Some(vec![0x68, (*imm & 255) as u8, ((*imm >> 8) & 255) as u8,
                                ((*imm >> 16) & 255) as u8, ((*imm >> 24) & 255) as u8])
                    }
                    (Sizes::Word, rm) | (Sizes::Long, rm)
                    | (Sizes::Quad, rm) if rm.is_rm() => {
                        assert!(R1::SIZE == Sizes::Quad);
                        let mut op = ByteCode::only_rm(rex, rm);
                        op.op_code = 0xff;
                        // op.small_reg_flag = R1::SIZE == Sizes::Word;
                        op.imm = Imm::NoImm(6);
                        Some(op.as_bytes())
                        // None
                    },
                    // Operand::LabRelAddr(_) => todo!(),
                    // Operand::LabAbsAddr(_) => todo!(),
                    // Operand::LabVal(_) => todo!(),
                    _ => None,
                }
            }
            InstrName::Pop => {
                let rm = self.reg1.as_ref().unwrap();
                assert!(rm.is_rm());
                assert!(self.reg2.is_none());
                assert!(R1::SIZE == Sizes::Quad);
                let mut op = ByteCode::only_rm(REX::new(), rm);
                op.op_code = 0x8F;
                Some(op.as_bytes())
            }
            InstrName::Not | InstrName::Neg // | InstrName::Mul
            | InstrName::UnsignedDiv | InstrName::SignedDiv => {
                let rm = self.reg1.as_ref().unwrap();
                assert!(rm.is_rm());
                assert!(self.reg2.is_none());
                let mut rex = REX::new();
                rex.w = R1::SIZE == Sizes::Quad;
                let mut op = ByteCode::only_rm(rex, rm);
                op.small_reg_flag = R1::SIZE == Sizes::Word;
                op.imm = Imm::NoImm(match self.instr {
                    InstrName::Not => 2,
                    InstrName::Neg => 3,
                    // InstrName::Mul => 4,
                    InstrName::UnsignedDiv => 6,
                    InstrName::SignedDiv => 7,
                    _ => panic!("Should not happen")
                });
                if R1::SIZE == Sizes::Byte {
                    op.op_code = 0xf6;
                } else {
                    op.op_code = 0xf7;
                }
                Some(op.as_bytes())
            }
            InstrName::Ret => Some(vec![0xc3]),
            InstrName::Leave => Some(vec![0xc9]),
            InstrName::Syscall => Some(vec![0x0f, 0x05]),
            InstrName::Hlt => Some(vec![0xf4]),
            InstrName::Cltd => Some(vec![0x99]),
            InstrName::Cqto => {
                let mut rex = REX::new();
                rex.w = true;
                Some(vec![rex.as_byte(), 0x99])
            },
            InstrName::Nop => Some(vec![0x90]),
            InstrName::Cmov(cond) => {
                assert_eq!(R1::SIZE, R2::SIZE);
                assert_ne!(R1::SIZE, Sizes::Byte);
                let rm = self.reg1.as_ref().unwrap();
                assert!(rm.is_rm());
                let reg = match self.reg2.as_ref().unwrap() {
                    Operand::Reg(reg) => reg,
                    _ => panic!("Not allowed"),
                };
                let op_code = match cond {
                    Cond::E | Cond::Z =>  0x44,
                    Cond::NE | Cond::NZ => 0x45,
                    Cond::S =>  0x48,
                    Cond::NS => 0x49,
                    Cond::G =>  0x4f,
                    Cond::GE => 0x4d,
                    Cond::L =>  0x4c,
                    Cond::LE => 0x4e,
                    Cond::A =>  0x47,
                    Cond::AE => 0x43,
                    Cond::B =>  0x42,
                    Cond::BE => 0x46,
                };
                let mut rex = REX::new();
                rex.w = R1::SIZE == Sizes::Quad;
                let mut op = ByteCode::new(rex, op_code, reg.clone(), rm);
                op.prefix = Some(0x0f);
                op.small_reg_flag = R1::SIZE == Sizes::Word;
                Some(op.as_bytes())
            }
            InstrName::Call(_) => None,
            InstrName::CallStar => None,
            InstrName::CondJump(_, _) => None,
            InstrName::Jump(_) => None,
            InstrName::JumpStar => None,
            InstrName::Set(cond) => {
                assert_eq!(R1::SIZE, Sizes::Byte);
                let rm = self.reg1.as_ref().unwrap();
                assert!(rm.is_rm());
                assert!(self.reg2.is_none());
                let rex = REX::new();
                let mut op = ByteCode::only_rm(rex, rm);
                op.prefix = Some(0x0f);
                op.op_code = match cond {
                    Cond::E | Cond::Z =>  0x94,
                    Cond::NE | Cond::NZ => 0x95,
                    Cond::S =>  0x98,
                    Cond::NS => 0x99,
                    Cond::G =>  0x9f,
                    Cond::GE => 0x9d,
                    Cond::L =>  0x9c,
                    Cond::LE => 0x9e,
                    Cond::A =>  0x97,
                    Cond::AE => 0x93,
                    Cond::B =>  0x92,
                    Cond::BE => 0x96,
                };
                Some(op.as_bytes())
            },
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

impl<S1: Reg, S2: Reg> Instruction<S1, S2> {
    fn default_writer(&self, file: &mut std::fs::File) -> std::io::Result<()> {
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

#[cfg(feature = "gen_binary")]
impl<S1: Reg, S2: Reg> InstrTrait for Instruction<S1, S2> {
    fn write_instr(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self.to_bin() {
            Some(v) => {
                // file.write_all("##".as_bytes())?;
                // self.default_writer(file)?;
                for b in v {
                    file.write_all(format!("\n\t.byte 0x{b:x}").as_bytes())?;
                }
                std::io::Result::Ok(())
            }
            None => self.default_writer(file),
        }
    }
}

#[cfg(not(feature = "gen_binary"))]
impl<S1: Reg, S2: Reg> InstrTrait for Instruction<S1, S2> {
    fn write_instr(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.default_writer(file)
    }
}

/// Type representing an instruction
pub type Instr = Box<dyn InstrTrait>;

impl Writable for Instr {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.write_instr(file)
    }
}
