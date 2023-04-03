use crate::traits::Reg;
use std::io::prelude::*;

/// Different operand sizes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sizes {
    /// 1 byte
    Byte,
    /// 2 bytes
    Word,
    /// 4 bytes
    Long,
    /// 8 bytes
    Quad,
    /// Invalid size
    Invalid,
}

impl Sizes {
    /// Get char related to size for instructions
    pub fn to_char(&self) -> char {
        match self {
            Self::Byte => 'b',
            Self::Word => 'w',
            Self::Long => 'l',
            Self::Quad => 'q',
            Self::Invalid => panic!("Internal error"),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 8 bytes registers
pub enum RegQ {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl RegQ {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Rax => "%rax",
            Self::Rbx => "%rbx",
            Self::Rcx => "%rcx",
            Self::Rdx => "%rdx",
            Self::Rsi => "%rsi",
            Self::Rdi => "%rdi",
            Self::Rbp => "%rbp",
            Self::Rsp => "%rsp",
            Self::R8 => "%r8",
            Self::R9 => "%r9",
            Self::R10 => "%r10",
            Self::R11 => "%r11",
            Self::R12 => "%r12",
            Self::R13 => "%r13",
            Self::R14 => "%r14",
            Self::R15 => "%r15",
        }
    }
}

impl Reg for RegQ {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(self.to_str().as_bytes())
    }

    fn to_bits(&self) -> (bool, u8) {
        match self {
            RegQ::Rax => (false, 0b000),
            RegQ::Rbx => (false, 0b011),
            RegQ::Rcx => (false, 0b001),
            RegQ::Rdx => (false, 0b010),
            RegQ::Rsi => (false, 0b110),
            RegQ::Rdi => (false, 0b111),
            RegQ::Rbp => (false, 0b101),
            RegQ::Rsp => (false, 0b100),
            RegQ::R8 => (true, 0b000),
            RegQ::R9 => (true, 0b001),
            RegQ::R10 => (true, 0b010),
            RegQ::R11 => (true, 0b011),
            RegQ::R12 => (true, 0b100),
            RegQ::R13 => (true, 0b101),
            RegQ::R14 => (true, 0b110),
            RegQ::R15 => (true, 0b111),
        }
    }

    const SIZE: Sizes = Sizes::Quad;
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 4 bytes registers
pub enum RegL {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

impl RegL {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Eax => "%eax",
            Self::Ebx => "%ebx",
            Self::Ecx => "%ecx",
            Self::Edx => "%edx",
            Self::Esi => "%esi",
            Self::Edi => "%edi",
            Self::Ebp => "%ebp",
            Self::Esp => "%esp",
            Self::R8d => "%r8d",
            Self::R9d => "%r9d",
            Self::R10d => "%r10d",
            Self::R11d => "%r11d",
            Self::R12d => "%r12d",
            Self::R13d => "%r13d",
            Self::R14d => "%r14d",
            Self::R15d => "%r15d",
        }
    }
}

impl Reg for RegL {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(self.to_str().as_bytes())
    }

    fn to_bits(&self) -> (bool, u8) {
        match self {
            Self::Eax => (false, 0b000),
            Self::Ebx => (false, 0b011),
            Self::Ecx => (false, 0b001),
            Self::Edx => (false, 0b010),
            Self::Esi => (false, 0b110),
            Self::Edi => (false, 0b111),
            Self::Ebp => (false, 0b101),
            Self::Esp => (false, 0b100),
            Self::R8d => (true, 0b000),
            Self::R9d => (true, 0b001),
            Self::R10d => (true, 0b010),
            Self::R11d => (true, 0b011),
            Self::R12d => (true, 0b100),
            Self::R13d => (true, 0b101),
            Self::R14d => (true, 0b110),
            Self::R15d => (true, 0b111),
        }
    }

    const SIZE: Sizes = Sizes::Long;
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 2 bytes registers
pub enum RegW {
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,
}

impl RegW {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Ax => "%ax",
            Self::Bx => "%bx",
            Self::Cx => "%cx",
            Self::Dx => "%dx",
            Self::Si => "%si",
            Self::Di => "%di",
            Self::Bp => "%bp",
            Self::Sp => "%sp",
            Self::R8w => "%r8w",
            Self::R9w => "%r9w",
            Self::R10w => "%r10w",
            Self::R11w => "%r11w",
            Self::R12w => "%r12w",
            Self::R13w => "%r13w",
            Self::R14w => "%r14w",
            Self::R15w => "%r15w",
        }
    }
}

impl Reg for RegW {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(self.to_str().as_bytes())
    }

    fn to_bits(&self) -> (bool, u8) {
        match self {
            RegW::Ax => (false, 0b000),
            RegW::Cx => (false, 0b001),
            RegW::Dx => (false, 0b010),
            RegW::Bx => (false, 0b011),
            RegW::Sp => (false, 0b100),
            RegW::Bp => (false, 0b101),
            RegW::Si => (false, 0b110),
            RegW::Di => (false, 0b101),
            RegW::R8w => (true, 0b000),
            RegW::R9w => (true, 0b001),
            RegW::R10w => (true, 0b010),
            RegW::R11w => (true, 0b011),
            RegW::R12w => (true, 0b100),
            RegW::R13w => (true, 0b101),
            RegW::R14w => (true, 0b110),
            RegW::R15w => (true, 0b111),
        }
    }

    const SIZE: Sizes = Sizes::Word;
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 1 bytes registers
pub enum RegB {
    Al,
    Ah,
    Bl,
    Bh,
    Cl,
    Ch,
    Dl,
    Dh,
    Sil,
    Dil,
    Bpl,
    Spl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

impl RegB {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Al => "%al",
            Self::Ah => "%ah",
            Self::Bl => "%bl",
            Self::Bh => "%bh",
            Self::Cl => "%cl",
            Self::Ch => "%ch",
            Self::Dl => "%dl",
            Self::Dh => "%dh",
            Self::Sil => "%sil",
            Self::Dil => "%dil",
            Self::Bpl => "%bpl",
            Self::Spl => "%spl",
            Self::R8b => "%r8b",
            Self::R9b => "%r9b",
            Self::R10b => "%r10b",
            Self::R11b => "%r11b",
            Self::R12b => "%r12b",
            Self::R13b => "%r13b",
            Self::R14b => "%r14b",
            Self::R15b => "%r15b",
        }
    }
}

impl Reg for RegB {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(self.to_str().as_bytes())
    }

    fn to_bits(&self) -> (bool, u8) {
        match self {
            RegB::Al => (false, 0b000),
            RegB::Cl => (false, 0b001),
            RegB::Dl => (false, 0b010),
            RegB::Bl => (false, 0b011),
            RegB::Ah => todo!(),
            RegB::Spl => todo!(),
            RegB::Ch => todo!(),
            RegB::Bpl => todo!(),
            RegB::Dh => todo!(),
            RegB::Sil => todo!(),
            RegB::Bh => todo!(),
            RegB::Dil => todo!(),
            RegB::R8b => (true, 0b000),
            RegB::R9b => (true, 0b001),
            RegB::R10b => (true, 0b010),
            RegB::R11b => (true, 0b011),
            RegB::R12b => (true, 0b100),
            RegB::R13b => (true, 0b101),
            RegB::R14b => (true, 0b110),
            RegB::R15b => (true, 0b111),
        }
    }

    const SIZE: Sizes = Sizes::Byte;
}

#[derive(Debug, Clone)]
/// Type representing the various operands
pub enum Operand<T: Reg> {
    /// Address at 1 + 2 + 3 * 4
    Addr(i64, RegQ, Option<RegQ>, u8),
    /// Direct access to register
    Reg(T),
    /// Label relative to RIP
    LabRelAddr(Label),
    /// Absolute label address
    LabAbsAddr(Label),
    /// Get label value
    LabVal(Label),

    /// Immediate value
    Imm(i64),
}

impl<T: Reg> Operand<T> {
    /// test if operand is register of memory
    pub fn is_rm(&self) -> bool {
        match self {
            Self::Addr(_, _, _, _) | Self::Reg(_) => true,
            _ => false,
        }
    }

    /// Write operand in file
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::Reg(reg) => reg.write_in(file),
            Self::Addr(offset, reg, index, scale) => {
                file.write_all(format!("{}(", offset).as_bytes())?;
                reg.write_in(file)?;
                match index {
                    None => file.write_all(b")"),
                    Some(index) => {
                        file.write_all(b", ")?;
                        index.write_in(file)?;
                        file.write_all(format!(", {})", scale).as_bytes())
                    }
                }
            }
            Self::LabRelAddr(label) => {
                label.write_in(file)?;
                file.write_all(b"(%rip)")
            }
            Self::LabAbsAddr(label) => label.write_in(file),
            Self::LabVal(label) => {
                file.write_all(b"$")?;
                label.write_in(file)
            }
            Self::Imm(id) => file.write_all(format!("${}", id).as_bytes()),
        }
    }
}

#[derive(Debug, Clone)]
/// Label
pub struct Label {
    name: String,
}

impl Label {
    /// Create label from string
    pub fn from_str(name: String) -> Self {
        Self { name }
    }

    /// Printf function label
    pub fn printf() -> Self {
        Self {
            name: "printf".to_string(),
        }
    }

    /// Malloc function label
    pub fn malloc() -> Self {
        Self {
            name: "malloc".to_string(),
        }
    }

    /// Free function label
    pub fn free() -> Self {
        Self {
            name: "free".to_string(),
        }
    }

    /// Realloc function label
    pub fn realloc() -> Self {
        Self {
            name: "realloc".to_string(),
        }
    }

    #[doc(hidden)]
    pub fn panic() -> Self {
        Self {
            name: "panic".to_string(),
        }
    }

    #[cfg(target_os = "macos")]
    /// Write label in file (implementation differs on linux and mac)
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(b"_")?;
        file.write_all(self.name.as_bytes())
    }

    #[cfg(target_os = "linux")]
    /// Write label in file (implementation differs on linux and mac)
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        file.write_all(self.name.as_bytes())
    }
}

/// Type representing a register that can never occur
/// It is used internally to type instruction taking only a few operands
#[derive(Debug, Copy, Clone)]
pub enum RegInv {}

impl Reg for RegInv {
    const SIZE: Sizes = Sizes::Invalid;

    fn write_in(&self, _: &mut std::fs::File) -> std::io::Result<()> {
        panic!("Internal error")
    }

    fn to_bits(&self) -> (bool, u8) {
        panic!("Should not happen")
    }
}
